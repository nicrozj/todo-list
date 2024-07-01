use cercis::prelude::*;
use chrono::{Datelike, NaiveDate};

use super::components::*;
use self::components::WeekDay;

mod components;

pub fn home_view(date: NaiveDate) -> String {
    const SEARCH_TW: &str = "bg-stone-200 w-full border-4 border-stone-300
        rounded-full px-4 font-bold focus:border-stone-400";

    let content = rsx!(Page {
        title: "Обзор",

        main {
            class: "w-full md:w-[48rem] mx-auto px-4 pt-12",

            section {
                class: "flex {SEARCH_TW}",

                input {
                    class: "outline-none bg-transparent grow",

                    placeholder: "Найти..."
                }
                Icon {
                    class: "text-stone-400",

                    "search"
                }
            }
            section {
                class: "flex gap-2 sm:gap-4 mt-8 overflow-x-scroll rounded-xl",

                for i in 0..30 {
                    WeekDay {
                        day: i,
                        current: i + 1 == date.day() as usize
                    }
                }
            }
            section {
                hx_get: "/home/{date}/tasks",
                hx_trigger: "load",

                "Загрузка..."
            }
        }
    });

    content.render()
}
