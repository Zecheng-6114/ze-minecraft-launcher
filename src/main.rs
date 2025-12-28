use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "我的世界启动器",
        options,
        Box::new(|cc| {
            // --- 核心：自动加载系统字体 ---
            let mut font_db = fontdb::Database::new();
            font_db.load_system_fonts();

            // 存储找到的字体数据 (Vec<u8>) 和索引
            let mut font_data_to_add: Vec<(Vec<u8>, u32)> = Vec::new();
            let mut font_names = Vec::new();

            for face in font_db.faces() {
                let family_name = face.families.first().map(|f| f.0.as_str()).unwrap_or("");
                // 判断是否为常见中文字体
                if family_name.contains("Microsoft YaHei")
                    || family_name.contains("SimHei")
                    || family_name.contains("SimSun")
                    || family_name.contains("PingFang SC")
                    || family_name.contains("Source Han Sans SC")
                    || family_name.contains("WenQuanYi Micro Hei")
                    || family_name.contains("Noto Sans CJK SC")
                // 常见Linux字体
                {
                    // 关键修改：正确处理 fontdb 返回的数据类型
                    let font_bytes: Option<Vec<u8>> = match &face.source {
                        // fontdb::Source::Binary 包装的是 Arc<dyn AsRef<[u8]> + ...>
                        fontdb::Source::Binary(arc_data) => {
                            // 通过 AsRef<[u8]> 获取字节切片，然后转换为 Vec<u8>
                            let slice: &[u8] = arc_data.as_ref().as_ref();
                            Some(slice.to_vec())
                        }
                        fontdb::Source::File(path) => std::fs::read(path).ok(),
                        _ => None,
                    };

                    if let Some(data) = font_bytes {
                        // face.index 本身就是 u32，直接使用
                        font_data_to_add.push((data, face.index));
                        font_names.push(family_name.to_string());
                        // 找到一个就退出，保证启动速度
                        break;
                    }
                }
            }

            // 将找到的系统字体添加到 egui
            if !font_data_to_add.is_empty() {
                let mut fonts = egui::FontDefinitions::default();
                for (i, (data, index)) in font_data_to_add.iter().enumerate() {
                    let font_name = format!("sys_chinese_{}", i);
                    println!("加载字体 '{}', 索引: {}", font_names[i], index);

                    // 创建 FontData 并用 Arc 包装（egui 的 insert 方法需要 Arc<FontData>）
                    let font_data = std::sync::Arc::new(egui::FontData::from_owned(data.clone()));

                    fonts.font_data.insert(font_name.clone(), font_data);
                    // 将该字体设置为“比例字体”的首选
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_name);
                }
                cc.egui_ctx.set_fonts(fonts);
                println!("已加载系统字体: {:?}", font_names);
            } else {
                eprintln!("警告：未能在系统中找到常见的中文字体，中文可能仍显示为方块。");
                // 这里可以添加回退方案，例如加载内置的备用字体文件
            }
            // --- 字体加载结束 ---

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("我的世界启动器 - 测试中文");
            ui.label("如果这行文字正常显示，说明字体加载成功！");
            ui.separator();
            ui.label("这是一个使用 Rust + egui 开发的启动器原型。");
            if ui.button("启动游戏").clicked() {
                println!("游戏启动中...");
                // 在这里调用你的启动逻辑，例如 lyceris 库
            }
            if ui.button("检查更新").clicked() {
                println!("检查游戏更新...");
            }
        });
    }
}
