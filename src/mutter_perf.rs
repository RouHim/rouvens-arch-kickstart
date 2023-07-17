use crate::shell::RootShell;
use crate::{pacman, Feature};

#[derive(Clone)]
pub struct MutterPerformance {}

const PACKAGE_NAME: &str = "mutter-performance";

impl Feature for MutterPerformance {
    fn install(&self, root_shell: &mut RootShell) -> bool {
        // First uninstall the default mutter package
        pacman::uninstall("mutter", root_shell);

        // Then install the mutter-performance package
        pacman::install(PACKAGE_NAME, root_shell);

        MutterPerformance::is_installed(self)
    }

    fn uninstall(&self, root_shell: &mut RootShell) -> bool {
        // remove the mutter-performance package
        pacman::uninstall(PACKAGE_NAME, root_shell);

        // install the default mutter package
        pacman::install("mutter", root_shell);

        !MutterPerformance::is_installed(self)
    }

    fn is_installed(&self) -> bool {
        pacman::is_installed(PACKAGE_NAME)
    }

    fn get_name(&self) -> String {
        String::from("Install mutter performance")
    }
}
