# IFO-Chan-Pet  
  
IFO酱~  
基于 Tauri + Live2D 的极简小桌宠。窗口包含悬浮球，可交互切换Live2D 模型可显示/隐藏。  
  
## 功能  

- 鼠标穿透，悬浮球可拖拽和点击并与模型交互  
- 点击悬浮球切换 Live2D 模型的显示与隐藏  
- 安装包见`Release`目录  
  
## 模型展示  
![IFO-Chan](img/image.png)
  
## 环境复现  
  
- Node.js 24.14.0  
- rustc 1.94.0  
  
## 快速开始  
  
```bash  
# 安装前端依赖  
npm install  
  
# 运行  
npm run tauri dev  
  
# 生产版本  
npm run tauri build  
```  
  
## 致谢  
  
- [Tauri](https://github.com/tauri-apps) 项目框架  
- [tauri-plugin-polygon](https://github.com/houycth/tauri-plugin-polygon) 局部交互插件  
- [l2d](https://github.com/hacxy/l2d) l2d前端库  
  
> 另外，模型也可以在个人网站[CFITSec](https://cfitsec.cn)找到。  
Enjoy your IFO-Chan!