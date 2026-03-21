from dataclasses import dataclass
import os
from pathlib import Path

import githubkit
import httpx
import tqdm
import asyncio
import tarfile

REPO_OWNER = "GitSquared"
REPO_NAME = "node-geolite2-redist"
BRANCH = "master"
PATH = "redist"
GITHUB = githubkit.GitHub(
    os.environ.get("GITHUB_TOKEN", "")
)
DOWNLOAD_DIR = Path("./data")
ASSETS = Path("../dashboard/backend/assets/ipdb")
FILTERS = ("copyright.txt", "license.txt")
PROXY = os.environ.get("HTTP_PROXY", None)
TIMEOUT = 60
TQDM_LOCK = asyncio.Lock()


@dataclass
class File:
    download_url: str
    name: str
    size: int


async def list_files():
    files = (await GITHUB.rest.repos.async_get_content(
        owner=REPO_OWNER, repo=REPO_NAME, path=PATH, ref=BRANCH
    )).parsed_data
    assert isinstance(files, list)
    return sorted([
        File(
            download_url=file.download_url or "",
            name=file.name,
            size=file.size,
        )
        for file in files if file.name.lower() not in FILTERS
    ], key=lambda x: x.name)


async def download_file(file: File, pbar: tqdm.tqdm):
    with open(DOWNLOAD_DIR / file.name, "wb") as f:
        async with httpx.AsyncClient(proxy=PROXY).stream("GET", file.download_url, timeout=TIMEOUT) as stream:
            total = 0
            async for chunk in stream.aiter_bytes():
                f.write(chunk)
                async with TQDM_LOCK:
                    total += len(chunk)
                    pbar.update(len(chunk))
                    pbar.set_postfix_str(f"{file.name} {tqdm.tqdm.format_sizeof(total)}")
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

#if __name__ == "__main__":
async def main():
    unlink_all(DOWNLOAD_DIR)
    unlink_all(ASSETS)
    
    DOWNLOAD_DIR.mkdir(exist_ok=True, parents=True)
    ASSETS.mkdir(exist_ok=True, parents=True)

    files = await list_files()
    # 使用 tqdm.write 避免干扰进度条
    for file in files:
        tqdm.tqdm.write(f"{file.name} {tqdm.tqdm.format_sizeof(file.size)}")

    with tqdm.tqdm(total=sum(file.size for file in files), unit="B", unit_scale=True, unit_divisor=1024, interval=0.5) as pbar:
        await asyncio.gather(
            *[download_file(file, pbar) for file in files]
        )

    for file in DOWNLOAD_DIR.iterdir():
        # if file.name.endswith(".tar.gz"):
        if file.name.endswith(".tar.gz"):
            extract_mmdb(file, ASSETS)
        # untar_file(file, ASSETS)

    # cp to dashboard backend


if __name__ == "__main__":
    asyncio.run(main())