use std::fmt::Display;

#[derive(Clone, Copy, Default)]
enum Fg {
    #[default]
    Default = 39,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightGrey = 97,
}

impl Display for Fg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Clone, Copy, Default)]
enum Bg {
    #[default]
    Default = 49,
    Black = 40,
    Red = 41,
    Green = 42,
    Yellow = 43,
    Blue = 44,
    Magenta = 45,
    Cyan = 46,
    Grey = 47,
    BrightBlack = 100,
    BrightRed = 101,
    BrightGreen = 102,
    BrightYellow = 103,
    BrightBlue = 104,
    BrightMagenta = 105,
    BrightCyan = 106,
    BrightGrey = 107,
}

impl Display for Bg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Clone, Copy)]
enum Style {
    Reset = 0,
    Underline = 4,
    Bold = 1,
    Blink = 5,
    Reverse = 7,
    Concealed = 8,
    Faint = 2,
    Italic = 3,
    Crossedout = 9,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

struct StyleFmt<Value> {
    fg: Fg,
    bg: Bg,
    style: Option<Style>,
    value: Value,
}

impl<Value> StyleFmt<Value>
where
    Value: Display,
{
    fn fg(value: Value, fg: Fg) -> Self {
        StyleFmt {
            fg,
            bg: Bg::default(),
            style: None,
            value,
        }
    }

    fn bg(value: Value, bg: Bg) -> Self {
        StyleFmt {
            fg: Fg::default(),
            bg,
            style: None,
            value,
        }
    }

    fn style(value: Value, style: Style) -> Self {
        StyleFmt {
            fg: Fg::default(),
            bg: Bg::default(),
            style: Some(style),
            value,
        }
    }

    fn with_fg(mut self, fg: Fg) -> Self {
        self.fg = fg;
        self
    }

    fn with_bg(mut self, bg: Bg) -> Self {
        self.bg = bg;
        self
    }

    fn with_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

impl<Value> Display for StyleFmt<Value>
where
    Value: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{}", self.fg, self.bg)?;
        if let Some(style) = self.style {
            write!(f, ";{}", style)?;
        };
        write!(
            f,
            "m{}\x1b[{};{};{}m",
            self.value,
            Fg::default() as u8,
            Bg::default() as u8,
            Style::Reset
        )
    }
}

pub trait TermStyle
where
    Self: Display + Sized,
{
    fn fg_black(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Black)
    }
    fn fg_red(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Red)
    }
    fn fg_green(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Green)
    }
    fn fg_yellow(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Yellow)
    }
    fn fg_blue(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Blue)
    }
    fn fg_magenta(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Magenta)
    }
    fn fg_cyan(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Cyan)
    }
    fn fg_grey(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::Grey)
    }
    fn fg_bright_black(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightBlack)
    }
    fn fg_bright_red(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightRed)
    }
    fn fg_bright_green(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightGreen)
    }
    fn fg_bright_yellow(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightYellow)
    }
    fn fg_bright_blue(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightBlue)
    }
    fn fg_bright_magenta(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightMagenta)
    }
    fn fg_bright_cyan(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightCyan)
    }
    fn fg_bright_grey(self) -> impl TermStyle {
        StyleFmt::fg(self, Fg::BrightGrey)
    }

    fn bg_black(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Black)
    }
    fn bg_red(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Red)
    }
    fn bg_green(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Green)
    }
    fn bg_yellow(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Yellow)
    }
    fn bg_blue(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Blue)
    }
    fn bg_magenta(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Magenta)
    }
    fn bg_cyan(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Cyan)
    }
    fn bg_grey(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::Grey)
    }
    fn bg_bright_black(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightBlack)
    }
    fn bg_bright_red(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightRed)
    }
    fn bg_bright_green(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightGreen)
    }
    fn bg_bright_yellow(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightYellow)
    }
    fn bg_bright_blue(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightBlue)
    }
    fn bg_bright_magenta(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightMagenta)
    }
    fn bg_bright_cyan(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightCyan)
    }
    fn bg_bright_grey(self) -> impl TermStyle {
        StyleFmt::bg(self, Bg::BrightGrey)
    }

    fn underline(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Underline)
    }
    fn bold(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Bold)
    }
    fn blink(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Blink)
    }
    fn reverse(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Reverse)
    }
    fn concealed(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Concealed)
    }
    fn faint(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Faint)
    }
    fn italic(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Italic)
    }
    fn crossedout(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Crossedout)
    }

    fn reset_style(self) -> impl TermStyle {
        StyleFmt::style(self, Style::Reset)
    }
}

impl<Value> TermStyle for StyleFmt<Value>
where
    Value: Display,
{
    fn fg_black(self) -> impl TermStyle {
        self.with_fg(Fg::Black)
    }

    fn fg_red(self) -> impl TermStyle {
        self.with_fg(Fg::Red)
    }

    fn fg_green(self) -> impl TermStyle {
        self.with_fg(Fg::Green)
    }

    fn fg_yellow(self) -> impl TermStyle {
        self.with_fg(Fg::Yellow)
    }

    fn fg_blue(self) -> impl TermStyle {
        self.with_fg(Fg::Blue)
    }

    fn fg_magenta(self) -> impl TermStyle {
        self.with_fg(Fg::Magenta)
    }

    fn fg_cyan(self) -> impl TermStyle {
        self.with_fg(Fg::Cyan)
    }

    fn fg_grey(self) -> impl TermStyle {
        self.with_fg(Fg::Grey)
    }

    fn fg_bright_black(self) -> impl TermStyle {
        self.with_fg(Fg::BrightBlack)
    }

    fn fg_bright_red(self) -> impl TermStyle {
        self.with_fg(Fg::BrightRed)
    }

    fn fg_bright_green(self) -> impl TermStyle {
        self.with_fg(Fg::BrightGreen)
    }

    fn fg_bright_yellow(self) -> impl TermStyle {
        self.with_fg(Fg::BrightYellow)
    }

    fn fg_bright_blue(self) -> impl TermStyle {
        self.with_fg(Fg::BrightBlue)
    }

    fn fg_bright_magenta(self) -> impl TermStyle {
        self.with_fg(Fg::BrightMagenta)
    }

    fn fg_bright_cyan(self) -> impl TermStyle {
        self.with_fg(Fg::BrightCyan)
    }

    fn fg_bright_grey(self) -> impl TermStyle {
        self.with_fg(Fg::BrightGrey)
    }

    fn bg_black(self) -> impl TermStyle {
        self.with_bg(Bg::Black)
    }

    fn bg_red(self) -> impl TermStyle {
        self.with_bg(Bg::Red)
    }

    fn bg_green(self) -> impl TermStyle {
        self.with_bg(Bg::Green)
    }

    fn bg_yellow(self) -> impl TermStyle {
        self.with_bg(Bg::Yellow)
    }

    fn bg_blue(self) -> impl TermStyle {
        self.with_bg(Bg::Blue)
    }

    fn bg_magenta(self) -> impl TermStyle {
        self.with_bg(Bg::Magenta)
    }

    fn bg_cyan(self) -> impl TermStyle {
        self.with_bg(Bg::Cyan)
    }

    fn bg_grey(self) -> impl TermStyle {
        self.with_bg(Bg::Grey)
    }

    fn bg_bright_black(self) -> impl TermStyle {
        self.with_bg(Bg::BrightBlack)
    }

    fn bg_bright_red(self) -> impl TermStyle {
        self.with_bg(Bg::BrightRed)
    }

    fn bg_bright_green(self) -> impl TermStyle {
        self.with_bg(Bg::BrightGreen)
    }

    fn bg_bright_yellow(self) -> impl TermStyle {
        self.with_bg(Bg::BrightYellow)
    }

    fn bg_bright_blue(self) -> impl TermStyle {
        self.with_bg(Bg::BrightBlue)
    }

    fn bg_bright_magenta(self) -> impl TermStyle {
        self.with_bg(Bg::BrightMagenta)
    }

    fn bg_bright_cyan(self) -> impl TermStyle {
        self.with_bg(Bg::BrightCyan)
    }

    fn bg_bright_grey(self) -> impl TermStyle {
        self.with_bg(Bg::BrightGrey)
    }

    fn underline(self) -> impl TermStyle {
        self.with_style(Style::Underline)
    }

    fn bold(self) -> impl TermStyle {
        self.with_style(Style::Bold)
    }

    fn blink(self) -> impl TermStyle {
        self.with_style(Style::Blink)
    }

    fn reverse(self) -> impl TermStyle {
        self.with_style(Style::Reverse)
    }

    fn concealed(self) -> impl TermStyle {
        self.with_style(Style::Concealed)
    }

    fn faint(self) -> impl TermStyle {
        self.with_style(Style::Faint)
    }

    fn italic(self) -> impl TermStyle {
        self.with_style(Style::Italic)
    }

    fn crossedout(self) -> impl TermStyle {
        self.with_style(Style::Crossedout)
    }

    fn reset_style(self) -> impl TermStyle {
        self.with_fg(Fg::default())
            .with_bg(Bg::default())
            .with_style(Style::Reset)
    }
}

impl<'a> TermStyle for &'a str {}
impl TermStyle for String {}
