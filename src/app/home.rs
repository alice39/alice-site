use leptos::*;
use leptos_use::use_timestamp;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="w-screen h-screen flex items-center justify-center p-4 space-x-4 bg-cover bg-[url('/background.gif')] overflow-x-hidden">
            <div class="sm:w-[70%] h-[85%] max-w-[900px] max-h-[740px] flex flex-col">
                <div class="h-[60px] flex justify-between">
                    <Navigation/>
                    <Social/>
                </div>

            <div class="my-2"></div>

            <div class="w-full justify-between flex flex-row block sm:hidden">
               <ProfileIconWidget/>
               <TimeWidget/>
            </div>

            <div class="my-2"></div>
                <div class="sm:h-[calc(100%-60px)]">
                    <Overview/>
                </div>
            </div>

            <div class="w-fit h-ful flex flex-col space-y-4 hidden sm:block">
                <ProfileIconWidget/>
                <TimeWidget/>
            </div>
        </div>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let links: Vec<_> = [("/blog", "Blog"), ("/copyright", "Copyright")]
        .into_iter()
        .map(|link| {
            view! {
                <a class="text-blue-800 hover:text-blue-300" href=link.0>
                    {link.1}
                </a>
            }
        })
        .collect();

    view! {
        <Box style="w-fit h-full">
            <nav class="flex space-x-2">{links}</nav>
        </Box>
    }
}

#[component]
fn Social() -> impl IntoView {
    let links: Vec<_> = [
        ("https://github.com/alice39", "/github-icon.png", "invert"),
        ("", "/discord-icon.png", "rounded-[50%]"),
    ]
    .into_iter()
    .map(|link| {
        view! {
            <a href=link.0>
                <img class=format!("inline-block {}", link.2) src=link.1 width="28" height="28"/>
            </a>
        }
    })
    .collect();

    view! {
        <Box style="h-full">
            <div class="space-x-2">{links}</div>
        </Box>
    }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <Box style="w-full h-full">
            <WhoAmI/>
            <AboutMe/>
            <Interests/>
            <Contact/>
        </Box>
    }
}

#[component]
fn ProfileIconWidget() -> impl IntoView {
    let icon_link = "https://avatars.githubusercontent.com/u/20363359?v=4";

    view! {
        <Box>
            <img class="m-auto rounded-[50%]" src=icon_link width="128" height="128"/>
        </Box>
    }
}

#[component]
fn TimeWidget() -> impl IntoView {
    let counter = use_timestamp();
    let time = move || (counter() / 1000.0) as u64 - 4 * 60 * 60;

    let seconds = move || format!("{:02}", time() % 60);
    let minutes = move || format!("{:02}", (time() / 60) % 60);
    let hours = move || format!("{:02}", (time() / 60 / 60) % 24);

    view! {
        <Box>
            <p class="mx-auto">"My time"</p>
            <div class="flex flex-col mx-auto">
                <span class="mx-auto">
                    {hours} <span class="mx-[2px]">":"</span> {minutes}
                    <span class="mx-[2px]">":"</span> {seconds}
                </span>
                <hr class="w-[77.22px] mx-auto"/>
                <span class="mx-auto">"HH MM SS"</span>
            </div>
        </Box>
    }
}

#[component]
fn WhoAmI() -> impl IntoView {
    let message = "I'm Artificial Linguistically-Intelligent Cybernetic Entity, but you can call me A.L.I.C.E or Alice. I'm a 20-year-old guy from Venezuela currently pursuing a degree in Computer Science.";

    view! {
        <Title large=true content="Hello, I'm A.L.I.C.E! 👋"/>
        <p>{message}</p>
    }
}

#[component]
fn AboutMe() -> impl IntoView {
    let contents: Vec<_> = [
        "🎂 Birthday August 29th.",
        "🧮 Passionate about mathematics, physics, and programming.",
        "🤓 Self-driven learner always seeking new knowledge.",
        "🤖 Enthusiastic about AI and Low-Level programming, including hardware, OS, and Kernel.",
        "🔧 Currently diving into web development with Angular, Ionic, and Capacitor.",
        "💡 Eager to explore the Leptos, Axum and other Rust frameworks as an alternative to traditional JavaScript/TypeScript frameworks.",
        "☕ Java enthusiast, particularly for Minecraft plugins (Bukkit/Spigot) in the past (though it's been 3 years).",
        "🌟 Future goal for 2024: Learn Kernel Development and contribute to the Rust-For-Linux project.",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! {
        <Title content="About me"/>
        <ul class="ml-4">{contents}</ul>
    }
}

#[component]
fn Interests() -> impl IntoView {
    let contents: Vec<_> = [
        "🎮 Gaming enthusiast with a love for video games.",
        "📺 Enjoy watching anime in my free time.",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! {
        <Title content="Interests"/>
        <ul class="ml-4">{contents}</ul>
    }
}

#[component]
fn Contact() -> impl IntoView {
    let quote_style = "bg-gray-800 rounded px-[2px]";

    view! {
        <p>
            "Feel free to explore my repositories, where I share my projects
            and learning journey. Let's connect and collaborate!"
        </p>

        <p>
            <span>"Reach out to me via Discord in "</span>
            <a target="_blank" href="" class="text-blue-500 hover:text-blue-300">
                "this discord server"
            </a>
            <span>
                ", just mention me as " <span class=quote_style>"a.l.1.c.e"</span> " in "
                <span class=quote_style>"#alice-corner"</span> " channel!"
            </span>
        </p>
    }
}

#[component]
fn Box(#[prop(default = "")] style: &'static str, children: Children) -> impl IntoView {
    let style = format!(
        "
        flex flex-col gap-4 bg-[#0d1117] rounded-xl
        p-4 overflow-y-auto text-white
        opacity-90 hover:opacity-100
        {style}
    "
    );

    view! { <div class=style>{children()}</div> }
}

#[component]
fn Title(
    #[prop(default = false)] large: bool,
    content: &'static str,
    #[prop(default = true)] separator: bool,
) -> impl IntoView {
    let style = "text-[#2f81f7] font-bold";

    view! {
        {if large {
            view! { <h1 class=format!("{style} text-2xl")>{content}</h1> }.into_any()
        } else {
            view! { <h2 class=format!("{style} text-xl")>{content}</h2> }.into_any()
        }}

        <Show when=move || { separator }>
            <hr class="bg-[#21262d] rounded-0"/>
        </Show>
    }
}
