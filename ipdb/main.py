from dataclasses import dataclass
import os
from pathlib import Path

import githubkit
import httpx
import tqdm
import asyncio
import tarfile

MMDB_REPO_OWNER = "GitSquared"
MMDB_REPO_NAME = "node-geolite2-redist"
MMDB_BRANCH = "master"
MMDB_PATH = "redist"
MMDB_FILTERS = ("copyright.txt", "license.txt", 
"geolite2-asn.mmdb.sha384",
"geolite2-asn.tar.gz",
"geolite2-asn.tar.gz.sha256",
"geolite2-city.mmdb.sha384",
"geolite2-city.tar.gz.sha256",
"geolite2-country.mmdb.sha384",
"geolite2-country.tar.gz",
"geolite2-country.tar.gz.sha256",
)

IP2REGION_OWNER = "lionsoul2014"
IP2REGION_NAME = "ip2region"
IP2REGION_PATH = "data"
IP2REGION_BRANCH = "master"
IP2REGION_FILTERS = ("ipv4_source.txt", "ipv6_source.txt", "sample")

MMDB_CHINA_REPO_OWNER = "alecthw"
MMDB_CHINA_REPO_NAME = "mmdb_china_ip_list"
MMDB_CHINA_FILTERS = (
    "country-lite.mmdb",
    "version"
)
GITHUB = githubkit.GitHub(
    os.environ.get("GITHUB_TOKEN", "")
)
DOWNLOAD_DIR = Path("./data")
ASSETS = Path("../dashboard/backend/assets/ipdb")
PROXY = os.environ.get("HTTP_PROXY", None)
TIMEOUT = 60
TQDM_LOCK = asyncio.Lock()
DOWNLOAD_LOCK = asyncio.Semaphore(4)

@dataclass
class File:
    download_url: str
    name: str
    size: int


async def list_mmdb_files():
    files = (await GITHUB.rest.repos.async_get_content(
        owner=MMDB_REPO_OWNER, repo=MMDB_REPO_NAME, path=MMDB_PATH, ref=MMDB_BRANCH
    )).parsed_data
    assert isinstance(files, list)
    return sorted([
        File(
            download_url=file.download_url or "",
            name=file.name,
            size=file.size,
        )
        for file in files if file.download_url and file.name.lower() not in MMDB_FILTERS
    ], key=lambda x: x.name)

async def download_ip2region_files():
    files = (await GITHUB.rest.repos.async_get_content(
        owner=IP2REGION_OWNER, repo=IP2REGION_NAME, path=IP2REGION_PATH, ref=IP2REGION_BRANCH
    )).parsed_data
    assert isinstance(files, list)
    return sorted([
        File(
            download_url=file.download_url or "",
            name=file.name,
            size=file.size,
        )
        for file in files if file.name.lower() not in IP2REGION_FILTERS and file.download_url
    ], key=lambda x: x.name)

async def download_file(file: File, pbar: tqdm.tqdm):
    with open(DOWNLOAD_DIR / file.name, "wb") as f:
        async with DOWNLOAD_LOCK:
            async with httpx.AsyncClient(proxy=PROXY, follow_redirects=True).stream("GET", file.download_url, timeout=TIMEOUT) as stream:
                total = 0
                async for chunk in stream.aiter_bytes():
                    f.write(chunk)
                    async with TQDM_LOCK:
                        total += len(chunk)
                        pbar.update(len(chunk))
                        pbar.set_postfix_str(f"{file.name} {tqdm.tqdm.format_sizeof(total)}/{tqdm.tqdm.format_sizeof(file.size)}")
    return DOWNLOAD_DIR / file.name

def extract_mmdb(tar_gz_path: Path, output_dir: Path):
    """从 tar.gz 中提取所有 .mmdb 文件到 output_dir"""
    with tarfile.open(tar_gz_path, "r:gz") as tar:
        for member in tar.getmembers():
            if member.name.endswith(".mmdb"):
                # 提取单个文件，并保持原始文件名
                tar.extract(member, path=output_dir, filter='data')
                # 如果包内文件在子目录中，可能需要移动文件
                extracted = output_dir / member.name
                if extracted.parent != output_dir:
                    extracted.rename(output_dir / extracted.name)
                # rmdir
                extracted.parent.rmdir()
                    

def unlink_all(path: Path):
    if not path.exists():
        return
    for file in path.iterdir():
        if file.is_file():
            file.unlink()
        elif file.is_dir():
            unlink_all(file)
            if not file.exists():
                continue
            file.rmdir()
    path.rmdir()

async def get_mmdb_china_files():
    # https://github.com/alecthw/mmdb_china_ip_list/releases
    resp = (await GITHUB.rest.repos.async_get_latest_release(owner=MMDB_CHINA_REPO_OWNER, repo=MMDB_CHINA_REPO_NAME)).parsed_data
    return [
        File(
            download_url=asset.browser_download_url,
            name=f"China_{asset.name}",
            size=asset.size,
        )
        for asset in resp.assets if asset.name.lower() not in MMDB_CHINA_FILTERS
    ]

#if __name__ == "__main__":
async def main():
    if PROXY:
        print(f"Proxy: {PROXY}")
    unlink_all(DOWNLOAD_DIR)
    unlink_all(ASSETS)
    
    DOWNLOAD_DIR.mkdir(exist_ok=True, parents=True)
    ASSETS.mkdir(exist_ok=True, parents=True)

    mmdb_files = await list_mmdb_files()
    #ip2region_files = await download_ip2region_files()
    mmdb_china_files = await get_mmdb_china_files()

    files = mmdb_files + mmdb_china_files
    # 使用 tqdm.write 避免干扰进度条
    for file in files:
        tqdm.tqdm.write(f"{file.name} {tqdm.tqdm.format_sizeof(file.size)} {file.download_url}")

    with tqdm.tqdm(total=sum(file.size for file in files), unit="B", unit_scale=True, unit_divisor=1024, mininterval=0.5) as pbar:
        await asyncio.gather(
            *[download_file(file, pbar) for file in files]
        )

    for file in DOWNLOAD_DIR.iterdir():
        # if file.name.endswith(".tar.gz"):
        if file.name.endswith(".tar.gz"):
            extract_mmdb(file, ASSETS)
        # untar_file(file, ASSETS)
        if file.name.endswith(".mmdb"):
            # copy to
            with open(file, "rb") as r, open(ASSETS / file.name, "wb") as w:
                while (chunk := r.read(16384)):
                    w.write(chunk)
            
            

    # cp to dashboard backend


if __name__ == "__main__":
    asyncio.run(main())