use crate::ascii_chars::AsciiChars;
use crate::stats::Stats;
use chrono::NaiveDateTime;
use leptos::task::spawn_local;
use leptos::{
    ev::{Event, SubmitEvent},
    html,
    prelude::*,
};
// use rand::Rng;
use crate::common::random_byte;
use crate::svg_keyboard::SvgKeyboard;
use crate::timer::Timer;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn rand_char() -> &'static AsciiChars {
    const LIST: [AsciiChars; 94] = AsciiChars::list();

    let index = random_byte(0, 93);
    &LIST[index as usize]
}

static STATS: OnceLock<Mutex<Stats>> = OnceLock::new();
static TIMER: OnceLock<Mutex<Timer>> = OnceLock::new();

static HIGHLIGHTED_KEYS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn add_hit(char_id: AsciiChars, timestamp: NaiveDateTime, reaction_time: u64) {
    if let Ok(mut stats) = STATS.get().unwrap().lock() {
        stats.add_hit(char_id, timestamp, reaction_time);
    }
}

fn add_miss(char_id: AsciiChars, timestamp: NaiveDateTime) {
    if let Ok(mut stats) = STATS.get().unwrap().lock() {
        stats.add_miss(char_id, timestamp);
    }
}

fn restart_timer() {
    if let Ok(mut timer) = TIMER.get().unwrap().lock() {
        timer.restart();
    }
}

fn stop_timer() -> u64 {
    if let Ok(mut timer) = TIMER.get().unwrap().lock() {
        timer.stop();
        timer.elapsed_ms()
    } else {
        0
    }
}

fn elapsed_timer() -> u64 {
    if let Ok(timer) = TIMER.get().unwrap().lock() {
        timer.elapsed_ms()
    } else {
        0
    }
}

fn add_highlighted_key(key: &str) {
    if let Some(lock) = HIGHLIGHTED_KEYS.get() {
        if let Ok(mut keys) = lock.lock() {
            keys.push(key.to_string());
        }
    }
}

fn remove_highlighted_key(key: &str) {
    if let Some(lock) = HIGHLIGHTED_KEYS.get() {
        if let Ok(mut keys) = lock.lock() {
            keys.retain(|k| k != key);
        }
    }
}

fn clear_highlighted_keys() {
    if let Some(lock) = HIGHLIGHTED_KEYS.get() {
        if let Ok(mut keys) = lock.lock() {
            keys.clear();
        }
    }
}

fn take_highlighted_key() -> Option<String> {
    if let Some(lock) = HIGHLIGHTED_KEYS.get() {
        if let Ok(mut keys) = lock.lock() {
            return keys.pop();
        }
    }
    None
}

#[component]
pub fn App() -> impl IntoView {
    STATS.set(Mutex::new(Stats::new())).unwrap();
    TIMER.set(Mutex::new(Timer::new_started())).unwrap();
    HIGHLIGHTED_KEYS.set(Mutex::new(Vec::new())).unwrap();

    // let (greet_msg, set_greet_msg) = signal(String::new());
    //
    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    let input_ref = NodeRef::<html::Input>::new();
    let hit_ref = NodeRef::<html::P>::new();
    let miss_ref = NodeRef::<html::P>::new();
    let report_ref = NodeRef::<html::Div>::new();
    let svg_keyboard_ref = NodeRef::<html::Div>::new();

    let (the_char, set_the_char) = signal(rand_char().as_char().to_string());
    let (keyboard_visible, set_keyboard_visible) = signal(false);

    let toggle_keyboard = move |_: Event| {
        set_keyboard_visible.update(|visible| *visible = !*visible);
    };

    let next_char = move || {
        let char = rand_char();
        set_the_char.set(char.as_char().to_string());
        svg_keyboard_ref.get().map(|el| {
            if let Some(doc) = el.owner_document() {
                // Remove previous highlights
                while let Some(key) = take_highlighted_key() {
                    if let Some(el2) = doc.get_element_by_id(&key) {
                        let _ = el2.class_list().remove_1("key-highlight");
                    }
                }
                // Highlight the new character
                let (kb_id, kb_shift) = SvgKeyboard::get_css_ids(&char);

                if let Some(el) = doc.get_element_by_id(&kb_id) {
                    add_highlighted_key(&kb_id);
                    let _ = el.class_list().add_1("key-highlight");
                }

                if let Some(kb_shift) = kb_shift {
                    if let Some(el) = doc.get_element_by_id(&kb_shift) {
                        add_highlighted_key(&kb_shift);
                        let _ = el.class_list().add_1("key-highlight");
                    }
                }
            }
        });
    };
    let (theirs, set_theirs) = signal(String::new());
    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }
    //
    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //         next_char();
    //     });
    // };

    // view! {
    //     <main class="container">
    //
    //         <form class="row" on:submit=greet>
    //             <input
    //                 id="greet-input"
    //                 placeholder="Enter a name..."
    //                 on:input=update_name
    //             />
    //             <button type="submit">"Greet"</button>
    //         </form>
    //         <p>{ move || greet_msg.get() }</p>
    //         <p>{ move || the_char.get() }</p>
    //     </main>
    // }

    // Focus the input on component mount
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });
    let check_result = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let want_char = the_char.get_untracked();
            if want_char.is_empty() {
                return;
            }

            let theirs = theirs.get_untracked();
            if theirs.is_empty() {
                return;
            }

            let their_char = theirs.chars().nth(0).unwrap();
            let want_char = want_char.chars().nth(0).unwrap();

            if their_char == want_char {
                let ms = stop_timer();
                let now = chrono::Local::now().naive_local();
                add_hit(AsciiChars::from_char(want_char).unwrap(), now, ms);

                next_char();
                // Clear DOM input
                if let Some(input) = input_ref.get() {
                    input.set_value("");
                }

                // Update hit count
                if let Some(hit) = hit_ref.get() {
                    let current_hits = STATS.get().unwrap().lock().unwrap().get_total_hit_count();
                    let current_hits = format!("Hits: {}", current_hits);

                    hit.set_inner_text(&current_hits);
                }
            } else {
                let now = chrono::Local::now().naive_local();
                add_miss(AsciiChars::from_char(want_char).unwrap(), now);

                if let Some(input) = input_ref.get() {
                    input.set_value("");
                }

                // Update misses count
                if let Some(miss) = miss_ref.get() {
                    let current_misses =
                        STATS.get().unwrap().lock().unwrap().get_total_miss_count();
                    let current_misses = format!("Misses: {}", current_misses);

                    miss.set_inner_text(&current_misses);
                }
            }

            restart_timer();

            //let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            //let new_msg = invoke("greet", args).await.as_string().unwrap();
            //set_greet_msg.set(new_msg);
        });
    };

    let update_theirs = move |ev| {
        let v = event_target_value(&ev);
        set_theirs.set(v.clone());

        // Auto-submit when a character is typed
        if !v.is_empty() {
            // Trigger the same logic as form submission
            spawn_local(async move {
                let want_char = the_char.get_untracked();
                if want_char.is_empty() {
                    return;
                }

                let theirs = theirs.get_untracked();
                if theirs.is_empty() {
                    return;
                }

                let their_char = theirs.chars().nth(0).unwrap();
                let want_char = want_char.chars().nth(0).unwrap();

                if their_char == want_char {
                    let ms = stop_timer();
                    let now = chrono::Local::now().naive_local();
                    add_hit(AsciiChars::from_char(want_char).unwrap(), now, ms);

                    next_char();
                    // Clear DOM input
                    if let Some(input) = input_ref.get() {
                        input.set_value("");
                    }

                    // Update hit count
                    if let Some(hit) = hit_ref.get() {
                        let current_hits =
                            STATS.get().unwrap().lock().unwrap().get_total_hit_count();
                        let current_hits = format!("Hits: {}", current_hits);

                        hit.set_inner_text(&current_hits);
                    }
                } else {
                    let now = chrono::Local::now().naive_local();
                    add_miss(AsciiChars::from_char(want_char).unwrap(), now);

                    if let Some(input) = input_ref.get() {
                        input.set_value("");
                    }

                    // Update misses count
                    if let Some(miss) = miss_ref.get() {
                        let current_misses =
                            STATS.get().unwrap().lock().unwrap().get_total_miss_count();
                        let current_misses = format!("Misses: {}", current_misses);

                        miss.set_inner_text(&current_misses);
                    }
                }

                restart_timer();
            });
        }
    };

    let update_report = move |_| {
        if let Some(report) = report_ref.get() {
            let stats = STATS.get().unwrap().lock().unwrap();
            let report_content = stats.generate_html_report();

            report.set_inner_html(&report_content);
        }
    };

    let reset_stats = move |_| {
        if let Ok(mut stats) = STATS.get().unwrap().lock() {
            stats.reset();
        }
        next_char();
        if let Some(hit) = hit_ref.get() {
            hit.set_inner_text("Hits: 0");
        }
        if let Some(miss) = miss_ref.get() {
            miss.set_inner_text("Misses: 0");
        }
        if let Some(report) = report_ref.get() {
            report.set_inner_html("");
        }
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
        restart_timer();
    };

    view! {
        <main class="container">
            <div class="toggle-container">
                <span class="toggle-label">"Keyboard"</span>
                <label class="toggle-switch">
                    <input type="checkbox" id="keyboard-toggle" on:change=toggle_keyboard/>
                    <span class="toggle-slider"></span>
                </label>
            </div>
            <h1>"Type the Character"</h1>
            <p id="want-input">{ move || the_char.get() }</p>
            <div node_ref=svg_keyboard_ref
                id="svg_keyboard"
                inner_html=SvgKeyboard::render()
                hidden=move || !keyboard_visible.get()></div>
            <form class="row" on:submit=check_result>
                <input
                    node_ref=input_ref
                    id="greet-input"
                    placeholder="Type the character..."
                    maxlength="1"
                    on:input=update_theirs
                />
            </form>
            <br/>
            <div>
              <button type="button" on:click=reset_stats>"Reset"</button>
              <button type="button" on:click=update_report>"Update Report"</button>
            </div>
            <p id="hits" node_ref=hit_ref></p>
            <p id="misses" node_ref=miss_ref></p>
            <div id="report"
                node_ref=report_ref></div>

        </main>
    }
}
// <button type="submit">"Greet"</button>
