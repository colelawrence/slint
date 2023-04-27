// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

slint::include_modules!();

pub fn main() {
    let main_window = MainWindow::new().unwrap();
    main_window.run().unwrap();
}
