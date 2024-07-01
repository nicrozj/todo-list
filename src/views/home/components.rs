use cercis::prelude::*;

const WEEK_DAYS: [&str; 7] = [
    "Пн",
    "Вт",
    "Ср",
    "Чт",
    "Пт",
    "Сб",
    "Вс",
];

#[component]
pub fn WeekDay(day: usize, current: bool) -> Element {
    let class = match current {
        true => "flex flex-col gap-1 bg-blue-300 rounded-xl text-center font-bold min-w-24 p-2",
        false => "flex flex-col gap-1 bg-stone-200 rounded-xl text-center min-w-24 p-2"
    };

    rsx!(div {
        class: "{class}",

        span { "{WEEK_DAYS[*day % 7]}" }
        span { "{day + 1}" }
    })
}
