use crate::actions::*;
use leptos::prelude::*;
use leptos_use::{UseTimeoutFnReturn, use_timeout_fn};

#[component]
pub fn Home() -> impl IntoView {
    let contact_me = ServerAction::<EmailMe>::new();
    let projects = vec![ProjectItem {
        title: "Github",
        description: "Github Profile where my public code lives",
        skills: vec!["Rust", "Javascript/Typesript", "Python", "Linux"],
        link: "https://github.com/cnava9389",
    }];
    view! {
        <section id="about">
            <div class="hero-section">
                <div class="bold-text blue-text text-lg padding-l">"About Me"</div>
                <p class="padding-x">"""
                    I'm a fullstack engineer with a strong foundation in Rust. My expertise in the Rust 
                    ecosystem includes Actix Web, for implementing scalable backends/web servers; 
                    Tokio, a rust async runtime for concurrent/parallel work loads; and Tauri, an alternative 
                    to Electron for building modern web and cross-platform applications. I'm also proficient in SQL 
                    (SQLite and PostgreSQL) for database management, and have working knowledge of Python, Go, JavaScript/TypeScript, 
                    Django, Node.js, and Deno. On the frontend, I'm experienced with React, WebSockets, Web Workers, 
                    and HTTP Streaming, but advocate for alternative frameworks like SolidJS, SvelteJS, and LeptosRS 
                    for their superior rendering performance. My development philosophy prioritizes simplicity, 
                    clear comments, rigorous testing, best practices, and continuous learning. Lastly, do not hesitate to reach out
                    enjoy this picture of my best friend!
                """</p>
                <div>
                    <button href="#contact" class="btn primary text-lg white-text">"Contact Me"</button>
                </div>
            </div>
            <div class="hero-section center">
                <img class="bobby" src="/assets/bobby.jpeg" />
            </div>
        </section>
        <section id="projects">
            <p class="bold-text blue-text text-lg center">"Projects"</p>
            <ProjectList projects />
        </section>
        <section id="skills">
            <p class="bold-text blue-text text-lg center">"Skills"</p>
            <div>
                <div class="hero-section center">
                    <p>"""
                        As a full-stack developer, I bring a comprehensive skill set to both front-end and back-end development.
                        Here's a glimpse into my technical expertise:
                    """</p> 
                </div>
                <div class="hero-section center">
                    <ul class="skills-list">
                        <li><b>"Languages: "</b>"Rust, JavaScript/TypeScript, Python, Go, CSS, HTML"</li>
                        <li><b>"Runtimes: "</b>"Tokio, Node.js, Deno, WASM"</li>
                        <li><b>"Frameworks: "</b>"Express.js, Django, Flask, Actix-web, Tauri, Next.js, React, Svelte-Kit, SCSS"</li>
                        <li><b>"Databases: "</b>"PostgreSQL, Sqlite, Redis, SurrealDB, MongoDB"</li>
                        <li><b>"Cloud: "</b>"AWS, Docker, Podman, Linux"</li>
                        <li><b>"Tools: "</b>"Git, CI/CD, Nix/NixOS"</li>
                    </ul>
                </div>
            </div>
        </section>
        <section id="contact">
            <div class="bold-text blue-text text-lg">"Contact Me"</div>
            <div class="center contact-card">
                <div class="text-center">"Feel free to reach out to me for any questions, collaborations, or opportunities."</div>
                <ActionForm action=contact_me>
                    <label>
                        "Name"
                        <input name="name" type="text" placeholder="Your Name" />
                    </label>
                    <label>
                        "Email"
                        <input name="email" type="email" placeholder="Your Email" required />
                    </label>
                    <label>
                        "Message"
                        <textarea name="message" placeholder="Your Message" />
                    </label>
                    <button type="submit" class="btn primary white-text text-lg">"Send Message"</button>
                </ActionForm>
            </div>
        </section>
        <footer class="center">
            <div>
                "copyright 2025 Portfolio. All rights reserved."
            </div>
        </footer>
    }
}

#[component]
pub fn Not_Found() -> impl IntoView {
    view! {
        <div>
        "Not Found"
        </div>
    }
}

#[derive(Clone)]
struct ProjectItem {
    title: &'static str,
    description: &'static str,
    skills: Vec<&'static str>,
    link: &'static str,
}

#[component]
fn ProjectList(projects: Vec<ProjectItem>) -> impl IntoView {
    let (right, set_right) = signal(false);
    let (left, set_left) = signal(false); 

    let (index, set_index) = signal(0);
    let (projects, _) = signal(projects);
    let right_index = move || if (index.get() + 1) >= projects.get().len() { 0 } else { index.get() + 1 };
    let left_index = move || if index.get() ==  0 { projects.get().len() - 1 } else { index.get() - 1 };

    let UseTimeoutFnReturn {
        start, is_pending, ..
    } = use_timeout_fn(
        move |index: usize| {
            set_left.set(false);
            set_right.set(false);
            set_index.set(index);
        },
        500.0,
    );
    let c_start = start.clone();

    view! {
        <div class="projects-list">
            <div on:click=move |_ev| if !is_pending.get() {
                set_left.set(true);
                start(left_index());
            } >
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-chevron-left" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M11.354 1.646a.5.5 0 0 1 0 .708L5.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0"/>
                </svg>
            </div>
            <div class:entering-right=left class:none=move || { right.get() || (!right.get() && !left.get()) } class="card">
                <h3>{move || projects.get()[left_index()].title}</h3>
                <div>{move || projects.get()[left_index()].description}</div>
                <div class="skills"> 
                    {move || {
                        projects.get()[left_index()].skills.clone().into_iter().map(|skill| {
                            view! {
                                <p>{skill}</p>
                            }
                        })
                        .collect::<Vec<_>>()
                    }}
                </div>
                <a class:clear class:blue-text class:text-lg target="_blank" rel="noopener noreferrer" href=move|| projects.get()[left_index()].link>"View Details"</a>
            </div>
            <div class:none=move || { right.get() || left.get() } class="card">
                <h3>{move || projects.get()[index.get()].title}</h3>
                <div>{move || projects.get()[index.get()].description}</div>
                <div class="skills">
                    {move || {
                        projects.get()[index.get()].skills.clone().into_iter().map(|skill| {
                            view! {
                                <p>{skill}</p>
                            }
                        })
                        .collect::<Vec<_>>()
                    }}
                </div>
                <a class:clear class:blue-text class:text-lg target="_blank" rel="noopener noreferrer" href=move|| projects.get()[index.get()].link>"View Details"</a>
            </div>
            <div class:entering-left=right class:none=move || { left.get() || (!right.get() && !left.get()) } class="card">
                <h3>{move || projects.get()[right_index()].title}</h3>
                <div>{move || projects.get()[right_index()].description}</div>
                <div class="skills"> 
                    {move || {
                        projects.get()[right_index()].skills.clone().into_iter().map(|skill| {
                            view! {
                                <p>{skill}</p>
                            }
                        })
                        .collect::<Vec<_>>()
                    }}
                </div>
                <a class:clear class:blue-text class:text-lg target="_blank" rel="noopener noreferrer" href=move|| projects.get()[right_index()].link>"View Details"</a>
            </div>
            <div on:click=move |_ev| if !is_pending.get() {
                set_right.set(true);
                c_start(right_index());
            }>
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="bi bi-chevron-right" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708"/>
                </svg>
            </div>
        </div>
    }
}
