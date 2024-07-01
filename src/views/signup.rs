use cercis::prelude::*;
use crate::views::components::Page;


pub fn signup_view() -> String {
    let content = rsx!(Page {
        title: "Регистрация",

        main {
            class: "flex flex-col h-screen justify-center w-full md:w-[48rem] mx-auto px-8",

            form {
                hx_post: "/signup",
                class: "flex flex-col justify-center items-center",
                h1 {
                    class: "mb-4",
                    "Регистрация"
                }

                input {
                    class: "w-3/5 mb-4 placeholder: p-1",
                    name: "login",
                    placeholder: "Логин"
                }
                input {
                    class: "w-3/5 mb-4 placeholder: p-1",
                    name: "password",
                    r#type: "password",
                    placeholder: "Пароль"
                }

                input {
                    class: "w-3/5 mb-4 placeholder: p-1",
                    name: "repeat_password",
                    r#type: "password",
                    placeholder: "Повторите пароль"
                }

                button {
                    class: "w-40 justify-center align-middle inline-flex items-center px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-800 text-sm font-medium rounded-md",
                    r#type: "submit",
                    "Зарегистрироваться"
                }
            }
        }

    });

    content.render()
}
