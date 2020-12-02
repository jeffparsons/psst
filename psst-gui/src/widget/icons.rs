use druid::{kurbo::BezPath, widget::prelude::*, Affine, Color, Key, KeyOrValue, Size};

pub static HOME: SvgIcon = SvgIcon {
    svg_path: "M8.17377 3.16499C8.12724 3.12048 8.06534 3.09564 8.00095 3.09564C7.93657 3.09564 7.87467 3.12048 7.82814 3.16499L2.07501 8.66092C2.05058 8.68429 2.03114 8.71238 2.01787 8.74348C2.00461 8.77458 1.99779 8.80804 1.99782 8.84186L1.99689 14C1.99689 14.2652 2.10224 14.5195 2.28978 14.7071C2.47732 14.8946 2.73167 15 2.99689 15H6.00001C6.13262 15 6.2598 14.9473 6.35357 14.8535C6.44734 14.7598 6.50002 14.6326 6.50002 14.5V10.25C6.50002 10.1837 6.52635 10.1201 6.57324 10.0732C6.62012 10.0263 6.68371 9.99998 6.75002 9.99998H9.25002C9.31632 9.99998 9.37991 10.0263 9.42679 10.0732C9.47368 10.1201 9.50002 10.1837 9.50002 10.25V14.5C9.50002 14.6326 9.55269 14.7598 9.64646 14.8535C9.74023 14.9473 9.86741 15 10 15H13.0019C13.2671 15 13.5215 14.8946 13.709 14.7071C13.8965 14.5195 14.0019 14.2652 14.0019 14V8.84186C14.0019 8.80804 13.9951 8.77458 13.9818 8.74348C13.9686 8.71238 13.9491 8.68429 13.9247 8.66092L8.17377 3.16499Z M15.3409 7.62969L13.0034 5.39344V2C13.0034 1.86739 12.9508 1.74021 12.857 1.64645C12.7632 1.55268 12.636 1.5 12.5034 1.5H11.0034C10.8708 1.5 10.7437 1.55268 10.6499 1.64645C10.5561 1.74021 10.5034 1.86739 10.5034 2V3L8.69344 1.26937C8.52407 1.09812 8.27219 1 8.00001 1C7.72876 1 7.47751 1.09813 7.30813 1.26969L0.661262 7.62906C0.466887 7.81656 0.442512 8.125 0.619387 8.32812C0.663803 8.3794 0.718187 8.42109 0.77923 8.45068C0.840273 8.48027 0.906698 8.49712 0.974462 8.50022C1.04223 8.50332 1.10991 8.4926 1.1734 8.4687C1.23689 8.44481 1.29485 8.40825 1.34376 8.36125L7.82813 2.165C7.87466 2.12049 7.93656 2.09565 8.00094 2.09565C8.06533 2.09565 8.12723 2.12049 8.17376 2.165L14.6587 8.36125C14.7543 8.45286 14.8822 8.50285 15.0146 8.50028C15.1469 8.4977 15.2728 8.44276 15.3647 8.3475C15.5566 8.14875 15.5406 7.82062 15.3409 7.62969Z",
    orig_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};
pub static LIBRARY: SvgIcon = SvgIcon {
    svg_path: "M3.5 0V1.5H13V13L14.5 14V0H3.5Z M1.5 2.5V16L6.75 12.125L12 16V2.5H1.5Z",
    orig_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};
#[allow(dead_code)]
pub static SEARCH: SvgIcon = SvgIcon {
    svg_path: "M14.2716 13.1684L11.3313 10.2281C12.0391 9.28573 12.4213 8.13865 12.42 6.96C12.42 3.94938 9.97062 1.5 6.96 1.5C3.94938 1.5 1.5 3.94938 1.5 6.96C1.5 9.97062 3.94938 12.42 6.96 12.42C8.13865 12.4213 9.28573 12.0391 10.2281 11.3313L13.1684 14.2716C13.3173 14.4046 13.5114 14.4756 13.711 14.47C13.9105 14.4645 14.1004 14.3827 14.2415 14.2415C14.3827 14.1004 14.4645 13.9105 14.47 13.711C14.4756 13.5114 14.4046 13.3173 14.2716 13.1684ZM3.06 6.96C3.06 6.18865 3.28873 5.43463 3.71727 4.79328C4.14581 4.15192 4.7549 3.65205 5.46753 3.35687C6.18017 3.06169 6.96433 2.98446 7.72085 3.13494C8.47738 3.28542 9.17229 3.65686 9.71772 4.20228C10.2631 4.74771 10.6346 5.44262 10.7851 6.19915C10.9355 6.95567 10.8583 7.73983 10.5631 8.45247C10.2679 9.1651 9.76808 9.77419 9.12672 10.2027C8.48537 10.6313 7.73135 10.86 6.96 10.86C5.92604 10.8588 4.93478 10.4475 4.20365 9.71635C3.47253 8.98522 3.06124 7.99396 3.06 6.96Z",
    orig_size: Size::new(16.0, 16.0),
    op: PaintOp::Fill,
};
pub static BACK: SvgIcon = SvgIcon {
    svg_path: "M9.70711 0.292893C10.0976 0.683417 10.0976 1.31658 9.70711 1.70711L2.41421 9L9.70711 16.2929C10.0976 16.6834 10.0976 17.3166 9.70711 17.7071C9.31658 18.0976 8.68342 18.0976 8.29289 17.7071L0.292893 9.70711C-0.0976311 9.31658 -0.0976311 8.68342 0.292893 8.29289L8.29289 0.292893C8.68342 -0.0976311 9.31658 -0.0976311 9.70711 0.292893Z",
    orig_size: Size::new(10.0, 18.0),
    op: PaintOp::Fill,
};
pub static PLAY: SvgIcon = SvgIcon {
    svg_path: "M4.92623 21.4262L19.9262 12.4262L4.92623 3.42623V21.4262Z",
    orig_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};
pub static PAUSE: SvgIcon = SvgIcon {
    svg_path: "M10.9262 20.6762H7.17623V4.17623H10.9262V20.6762ZM17.6762 20.6762H13.9262V4.17623H17.6762V20.6762Z",
    orig_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};
pub static SKIP_BACK: SvgIcon = SvgIcon {
    svg_path: "M7.15139 3.42623V11.0912L19.9262 3.42623V21.4262L7.15139 13.7612V21.4262H4.92623V3.42623H7.15139Z",
    orig_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};
pub static SKIP_FORWARD: SvgIcon = SvgIcon {
    svg_path: "M17.7011 3.42623V11.0912L4.92623 3.42623V21.4262L17.7011 13.7612V21.4262H19.9262V3.42623H17.7011Z",
    orig_size: Size::new(24.0, 24.0),
    op: PaintOp::Fill,
};
#[allow(dead_code)]
pub static CONFIG: SvgIcon = SvgIcon {
    svg_path: "M0 2.76923C0 2.25943 0.413276 1.84615 0.923077 1.84615H14.7692C15.279 1.84615 15.6923 2.25943 15.6923 2.76923C15.6923 3.27903 15.279 3.69231 14.7692 3.69231H0.923077C0.413276 3.69231 0 3.27903 0 2.76923ZM17.5385 2.76923C17.5385 2.25943 17.9517 1.84615 18.4615 1.84615H23.0769C23.5867 1.84615 24 2.25943 24 2.76923C24 3.27903 23.5867 3.69231 23.0769 3.69231H18.4615C17.9517 3.69231 17.5385 3.27903 17.5385 2.76923ZM0 10.1538C0 9.64404 0.413276 9.23077 0.923077 9.23077H5.53846C6.04826 9.23077 6.46154 9.64404 6.46154 10.1538C6.46154 10.6636 6.04826 11.0769 5.53846 11.0769H0.923077C0.413276 11.0769 0 10.6636 0 10.1538ZM8.30769 10.1538C8.30769 9.64404 8.72097 9.23077 9.23077 9.23077H23.0769C23.5867 9.23077 24 9.64404 24 10.1538C24 10.6636 23.5867 11.0769 23.0769 11.0769H9.23077C8.72097 11.0769 8.30769 10.6636 8.30769 10.1538ZM0 17.5385C0 17.0287 0.413276 16.6154 0.923077 16.6154H14.7692C15.279 16.6154 15.6923 17.0287 15.6923 17.5385C15.6923 18.0483 15.279 18.4615 14.7692 18.4615H0.923077C0.413276 18.4615 0 18.0483 0 17.5385ZM17.5385 17.5385C17.5385 17.0287 17.9517 16.6154 18.4615 16.6154H23.0769C23.5867 16.6154 24 17.0287 24 17.5385C24 18.0483 23.5867 18.4615 23.0769 18.4615H18.4615C17.9517 18.4615 17.5385 18.0483 17.5385 17.5385Z M16.6154 1.84615C16.1056 1.84615 15.6923 2.25943 15.6923 2.76923C15.6923 3.27903 16.1056 3.69231 16.6154 3.69231C17.1252 3.69231 17.5385 3.27903 17.5385 2.76923C17.5385 2.25943 17.1252 1.84615 16.6154 1.84615ZM13.8462 2.76923C13.8462 1.23983 15.086 0 16.6154 0C18.1448 0 19.3846 1.23983 19.3846 2.76923C19.3846 4.29863 18.1448 5.53846 16.6154 5.53846C15.086 5.53846 13.8462 4.29863 13.8462 2.76923Z M7.38461 9.23077C6.87481 9.23077 6.46154 9.64404 6.46154 10.1538C6.46154 10.6636 6.87481 11.0769 7.38461 11.0769C7.89442 11.0769 8.30769 10.6636 8.30769 10.1538C8.30769 9.64404 7.89442 9.23077 7.38461 9.23077ZM4.61538 10.1538C4.61538 8.62444 5.85521 7.38461 7.38461 7.38461C8.91402 7.38461 10.1538 8.62444 10.1538 10.1538C10.1538 11.6832 8.91402 12.9231 7.38461 12.9231C5.85521 12.9231 4.61538 11.6832 4.61538 10.1538Z M16.6154 16.6154C16.1056 16.6154 15.6923 17.0287 15.6923 17.5385C15.6923 18.0483 16.1056 18.4615 16.6154 18.4615C17.1252 18.4615 17.5385 18.0483 17.5385 17.5385C17.5385 17.0287 17.1252 16.6154 16.6154 16.6154ZM13.8462 17.5385C13.8462 16.0091 15.086 14.7692 16.6154 14.7692C18.1448 14.7692 19.3846 16.0091 19.3846 17.5385C19.3846 19.0679 18.1448 20.3077 16.6154 20.3077C15.086 20.3077 13.8462 19.0679 13.8462 17.5385Z",
    orig_size: Size::new(26.0, 26.0),
    op: PaintOp::Fill,
};
pub static ERROR: SvgIcon = SvgIcon {
    svg_path: "M19.2 10.4817C19.2 5.62551 15.2265 1.59102 10.3839 1.59971C5.53112 1.60842 1.6 5.54505 1.6 10.3997V15.4332C1.6 15.4332 1.6 15.4332 1.6 15.4332C1.60002 15.9128 1.74375 16.3814 2.01265 16.7786C2.28155 17.1758 2.66329 17.4833 3.10863 17.6614L4.29256 18.1349L4.29275 18.135C4.66339 18.2833 4.99117 18.5217 5.24641 18.8287C5.50165 19.1357 5.67629 19.5015 5.7545 19.893L6.25594 22.3997H14.5448L14.5456 22.3997L15.0453 19.8938C15.1232 19.5019 15.2977 19.1356 15.5529 18.8282C15.8082 18.5209 16.1362 18.2821 16.5071 18.1335L16.5074 18.1334L17.6914 17.6599C18.1367 17.4818 18.5184 17.1743 18.7873 16.7771C19.0562 16.3799 19.2 15.9113 19.2 15.4317C19.2 15.4317 19.2 15.4317 19.2 15.4317V10.4817ZM10.3811 -0.000288492C16.1265 -0.0105985 20.8 4.7589 20.8 10.4817V15.4317C20.8 16.2311 20.5604 17.0122 20.1122 17.6741C19.6641 18.3361 19.0278 18.8486 18.2856 19.1455L18.2856 19.1455L17.1019 19.6188C16.9783 19.6684 16.8689 19.748 16.7839 19.8504C16.6988 19.9529 16.6406 20.075 16.6147 20.2056L16.1144 22.7137C16.0418 23.0769 15.8453 23.4036 15.5587 23.6381C15.2722 23.8724 14.9134 24.0002 14.5434 23.9997C14.5431 23.9997 14.5427 23.9997 14.5424 23.9997L6.25606 23.9997C6.25604 23.9997 6.25602 23.9997 6.256 23.9997C5.88616 23.9997 5.52773 23.8716 5.24169 23.6371C4.95563 23.4027 4.75964 23.0764 4.68706 22.7137L4.68704 22.7136L4.18554 20.2066L4.1855 20.2064C4.15943 20.0759 4.10122 19.954 4.01614 19.8517C3.93106 19.7493 3.8218 19.6699 3.69825 19.6204L2.51444 19.147L2.51437 19.147C1.77216 18.8501 1.13592 18.3376 0.687749 17.6756C0.239577 17.0137 2.63691e-05 16.2326 0 15.4332V10.3997C0 4.66236 4.64587 0.0100032 10.3811 -0.000288492Z M6.00001 12C5.33727 12 4.80001 12.5373 4.80001 13.2C4.80001 13.8627 5.33727 14.4 6.00001 14.4C6.66275 14.4 7.20001 13.8627 7.20001 13.2C7.20001 12.5373 6.66275 12 6.00001 12ZM3.20001 13.2C3.20001 11.6536 4.45361 10.4 6.00001 10.4C7.54641 10.4 8.80001 11.6536 8.80001 13.2C8.80001 14.7464 7.54641 16 6.00001 16C4.45361 16 3.20001 14.7464 3.20001 13.2Z M14.8 12C14.1373 12 13.6 12.5373 13.6 13.2C13.6 13.8627 14.1373 14.4 14.8 14.4C15.4627 14.4 16 13.8627 16 13.2C16 12.5373 15.4627 12 14.8 12ZM12 13.2C12 11.6536 13.2536 10.4 14.8 10.4C16.3464 10.4 17.6 11.6536 17.6 13.2C17.6 14.7464 16.3464 16 14.8 16C13.2536 16 12 14.7464 12 13.2Z M10.4 15.2C10.7444 15.2 11.0501 15.4203 11.159 15.747L11.959 18.147C12.0403 18.391 11.9994 18.6592 11.849 18.8678C11.6986 19.0764 11.4572 19.2 11.2 19.2H9.60001C9.34286 19.2 9.10138 19.0764 8.95102 18.8678C8.80066 18.6592 8.75975 18.391 8.84107 18.147L9.64107 15.747C9.74996 15.4203 10.0557 15.2 10.4 15.2ZM8.00001 20.8C8.44184 20.8 8.80001 21.1582 8.80001 21.6V23.2C8.80001 23.6418 8.44184 24 8.00001 24C7.55818 24 7.20001 23.6418 7.20001 23.2V21.6C7.20001 21.1582 7.55818 20.8 8.00001 20.8ZM10.4 20.8C10.8418 20.8 11.2 21.1582 11.2 21.6V23.2C11.2 23.6418 10.8418 24 10.4 24C9.95818 24 9.60001 23.6418 9.60001 23.2V21.6C9.60001 21.1582 9.95818 20.8 10.4 20.8ZM12.8 20.8C13.2418 20.8 13.6 21.1582 13.6 21.6V23.2C13.6 23.6418 13.2418 24 12.8 24C12.3582 24 12 23.6418 12 23.2V21.6C12 21.1582 12.3582 20.8 12.8 20.8Z",
    orig_size: Size::new(26.0, 26.0),
    op: PaintOp::Fill,
};

#[derive(Copy, Clone)]
pub enum PaintOp {
    Fill,
}

pub struct SvgIcon {
    svg_path: &'static str,
    orig_size: Size,
    op: PaintOp,
}

impl SvgIcon {
    pub fn scale(&self, to_size: impl Into<Size>) -> Icon {
        let to_size = to_size.into();
        let bez_path = BezPath::from_svg(self.svg_path).expect("Failed to parse SVG");
        let scale = Affine::scale_non_uniform(
            to_size.width / self.orig_size.width,
            to_size.height / self.orig_size.height,
        );
        Icon::new(self.op, bez_path, to_size, scale)
    }
}

pub const ICON_COLOR: Key<Color> = Key::new("app.icon-color");

pub struct Icon {
    op: PaintOp,
    bez_path: BezPath,
    size: Size,
    scale: Affine,
    color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(op: PaintOp, bez_path: BezPath, size: Size, scale: Affine) -> Self {
        Icon {
            op,
            bez_path,
            size,
            scale,
            color: ICON_COLOR.into(),
        }
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }
}

impl<T> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, _env: &Env) -> Size {
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        ctx.with_save(|ctx| {
            ctx.transform(self.scale);
            match self.op {
                PaintOp::Fill => ctx.fill(&self.bez_path, &color),
            }
        });
    }
}
