use conrod;
use conrod::Sizeable;

pub trait StandardButton: Sizeable {
    fn std_size(self) -> Self {
        self.w_h(200.0, 40.0)
    }

    fn std_height(self) -> Self {
        self.h(40.0)
    }
}

impl<'a> StandardButton for conrod::widget::toggle::Toggle<'a> {}
impl<'a, Show> StandardButton for conrod::widget::button::Button<'a, Show> where Show: conrod::widget::button::Show {}
