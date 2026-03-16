import 'virtual:svg-icons-register';

import { use } from 'echarts';
import { CanvasRenderer } from 'echarts/renderers';
import { BarChart } from 'echarts/charts';
import { GridComponent, TooltipComponent } from 'echarts/components';
use([CanvasRenderer, BarChart, GridComponent, TooltipComponent]);
