#![allow(non_snake_case)]
use dioxus::prelude::*;
use chrono::{Local, Timelike};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

// --- Data Structures ---

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TimeNote {
    content: String,
    is_locked: bool, // "Banked" experience vs planned
}

const SAVE_FILE: &str = "chronos_notes.json";

fn load_notes() -> HashMap<String, TimeNote> {
    if let Ok(data) = fs::read_to_string(SAVE_FILE) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        HashMap::new()
    }
}

fn save_notes(notes: &HashMap<String, TimeNote>) {
    if let Ok(data) = serde_json::to_string_pretty(notes) {
        let _ = fs::write(SAVE_FILE, data);
    }
}

// --- Styles (The "Luxury Gold" Theme) ---

fn main() {
    let cfg = dioxus::desktop::Config::default()
        .with_window(dioxus::desktop::WindowBuilder::new().with_title("Chronos Aeternum Plantacerium"));
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

fn App() -> Element {
    // State for current time
    let mut time = use_signal(|| Local::now());
    // State for notes: Map Date-Hour (YYYY-MM-DD-HH) to a Note
    let mut notes = use_signal(|| load_notes());
    // State for currently selected hour (0-23) to edit
    let mut selected_hour = use_signal(|| None::<u32>);

    // Save notes to disk whenever they change
    use_effect(move || {
        save_notes(&notes.read());
    });

    // Update time continuously (60fps for smooth "flow")
    use_future(move || async move {
        loop {
            time.set(Local::now());
            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        }
    });

    let t = time();
    let sub_second = t.nanosecond() as f64 / 1_000_000_000.0;
    let second_deg = (t.second() as f64 + sub_second) * 6.0;
    let minute_deg = (t.minute() as f64 + t.second() as f64 / 60.0) * 6.0;
    let hour_deg = (t.hour() % 12) as f64 * 30.0 + (t.minute() as f64 / 2.0);

    // Calculate "Life Earned" (Seconds passed today)
    let experience_points = t.num_seconds_from_midnight();
    let precise_experience = experience_points as f64 + sub_second;
    let day_progress = precise_experience / 86400.0;
    let stroke_dasharray = day_progress * 1507.0; // 2 * PI * 240
    let minute_progress = (t.minute() as f64 + t.second() as f64 / 60.0) / 60.0;
    // Minute Hand coordinates for the "Spirit Dot"
    let minute_angle = (minute_progress * 360.0).to_radians();
    let minute_dot_x = 400.0 + 230.0 * minute_angle.sin();
    let minute_dot_y = 400.0 - 230.0 * minute_angle.cos();

    let mut save_signal = use_signal(|| false);
    let display_modal_h = selected_hour().map(|h| if h == 0 { 12 } else { h }).unwrap_or(0);

    // Trigger save animation
    let on_save = move |_| {
        save_notes(&notes.read());
        save_signal.set(true);
        // Reset signal after 2 seconds
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            save_signal.set(false);
        });
    };

    rsx! {
        // Embed Critical CSS for guaranteed luxury rendering
        style {
            "
            @import url('https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Montserrat:wght@100;200;400;600&display=swap');
            
            :root {{
                --gold-primary: #D4AF37;
                --gold-light: #FCF6BA;
                --gold-dark: #AA771C;
                --gold-gradient: linear-gradient(135deg, #BF953F, #FCF6BA, #B38728, #FBF5B7, #AA771C);
            }}

            body {{ 
                margin: 0; padding: 0; background: #020202; 
                color: #FCF6BA; font-family: 'Montserrat', sans-serif;
                overflow: hidden;
            }}

            .viewport-center {{
                width: 100vw; height: 100vh;
                position: relative;
                display: flex; flex-direction: column; justify-content: center; align-items: center;
                background: radial-gradient(circle at center, #0a0a0a 0%, #000 100%);
            }}

            .mandala-layer {{
                position: absolute;
                top: 50%; left: 50%;
                transform: translate(-50%, -50%);
                z-index: 1;
                opacity: 0.3;
                animation: rotate-mandala-ccw 600s linear infinite;
                pointer-events: none;
            }}

            @keyframes rotate-mandala-ccw {{
                from {{ transform: translate(-50%, -50%) rotate(360deg); }}
                to {{ transform: translate(-50%, -50%) rotate(0deg); }}
            }}

            .watch-layer {{
                position: relative;
                z-index: 10;
                width: 850px; height: 850px;
                display: flex; justify-content: center; align-items: center;
                margin-top: 20px;
            }}

            .gold-text {{
                background: var(--gold-gradient);
                -webkit-background-clip: text;
                background-clip: text;
                -webkit-text-fill-color: transparent;
                filter: drop-shadow(0 0 15px rgba(212, 175, 55, 0.6));
            }}

            @keyframes breathe-glow {{
                0% {{ opacity: 0.4; stroke-width: 8; }}
                50% {{ opacity: 0.8; stroke-width: 15; }}
                100% {{ opacity: 0.4; stroke-width: 8; }}
            }}

            .rim-glow-breathe {{
                animation: breathe-glow 8s infinite ease-in-out;
            }}

            .luxury-btn {{
                background: linear-gradient(135deg, #AA771C, #FCF6BA, #AA771C);
                background-size: 200% 200%;
                border: none;
                padding: 12px 30px;
                font-family: 'Cinzel', serif;
                font-weight: 900;
                letter-spacing: 3px;
                cursor: pointer;
                transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
                color: #000;
                border-radius: 24px;
                box-shadow: 0 0 15px rgba(212, 175, 55, 0.15);
                text-transform: uppercase;
            }}

            .luxury-btn:hover {{
                background-position: 100% 0;
                transform: scale(1.05);
                box-shadow: 0 0 40px rgba(212, 175, 55, 0.4);
            }}

            @keyframes pulse-hub {{
                0% {{ transform: scale(0.92); opacity: 0.8; }}
                50% {{ transform: scale(1.08); opacity: 1; }}
                100% {{ transform: scale(0.92); opacity: 0.8; }}
            }}

            /* Dual Direction Emanation */
            .emanate-out {{
                position: absolute;
                top: 50%; left: 50%;
                transform: translate(-50%, -50%);
                border-radius: 50%;
                border: 1px solid rgba(212, 175, 55, 0.06);
                animation: emanate-out 8s infinite ease-out;
                pointer-events: none;
            }}

            .emanate-in {{
                position: absolute;
                top: 50%; left: 50%;
                transform: translate(-50%, -50%);
                border-radius: 50%;
                border: 1px solid rgba(212, 175, 55, 0.04);
                animation: emanate-in 10s infinite ease-in-out;
                pointer-events: none;
            }}

            @keyframes emanate-out {{
                0% {{ width: 100px; height: 100px; opacity: 0; transform: translate(-50%, -50%) scale(0.6); }}
                40% {{ opacity: 0.2; }}
                100% {{ width: 1400px; height: 1400px; opacity: 0; transform: translate(-50%, -50%) scale(1.1); }}
            }}

            @keyframes emanate-in {{
                0% {{ width: 1600px; height: 1600px; opacity: 0; transform: translate(-50%, -50%) scale(1.2); }}
                50% {{ opacity: 0.15; }}
                100% {{ width: 200px; height: 200px; opacity: 0; transform: translate(-50%, -50%) scale(0.5); }}
            }}

            .save-status {{
                position: fixed;
                top: 30px; right: 30px;
                padding: 18px 35px;
                background: rgba(10, 10, 10, 0.95);
                border: 1px solid var(--gold-dark);
                color: var(--gold-light);
                font-family: 'Cinzel', serif;
                font-weight: 700;
                letter-spacing: 2px;
                z-index: 1000;
                border-radius: 4px;
                box-shadow: 0 10px 40px rgba(0,0,0,0.8);
                animation: slide-in 0.6s cubic-bezier(0.23, 1, 0.32, 1);
            }}

            @keyframes slide-in {{
                from {{ transform: translateX(120%); opacity: 0; }}
                to {{ transform: translateX(0); opacity: 1; }}
            }}

            .markdown-body h1, .markdown-body h2, .markdown-body h3 {{
                font-family: 'Cinzel', serif;
                color: var(--gold-primary);
                letter-spacing: 2px;
                border-bottom: 2px solid rgba(212, 175, 55, 0.2);
                padding-bottom: 5px;
                margin-top: 20px;
            }}

            .markdown-body {{
                line-height: 1.8;
                color: #ccc;
            }}

            .markdown-body p {{
                margin-bottom: 15px;
            }}

            .markdown-body strong {{
                color: var(--gold-light);
            }}
            "
        }

        div { class: "viewport-center",
            
            // 1. Counter-Clockwise Mandala
            div { class: "mandala-layer",
                svg {
                    view_box: "0 0 1000 1000",
                    style: "width: 1300px; height: 1300px;",
                    defs {
                        linearGradient { id: "mandalaGold", x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                            stop { offset: "0%", stop_color: "#BF953F", stop_opacity: "0.4" }
                            stop { offset: "100%", stop_color: "#AA771C", stop_opacity: "0.05" }
                        }
                    }
                    // Mandala Lotus Geometry
                    for i in 0..32 {
                        {
                            let rotation = i as f64 * (360.0 / 32.0);
                            rsx! {
                                path {
                                    d: "M 500 500 C 520 400 580 400 600 500 C 580 600 520 600 500 500",
                                    fill: "none",
                                    stroke: "url(#mandalaGold)",
                                    stroke_width: "0.4",
                                    style: "transform-origin: 500px 500px; transform: rotate({rotation}deg) translate(0, -320px) scale(2.5, 1.5);"
                                }
                            }
                        }
                    }
                    circle { cx: "500", cy: "500", r: "485", fill: "none", stroke: "url(#mandalaGold)", stroke_width: "0.5", opacity: "0.15" }
                }
            }

            // 2. Centered Chronometer & Dual Emanations
            div { class: "watch-layer",
                
                // Outward rings
                for i in 0..3 {
                    {
                        let delay = i as f64 * 2.5;
                        rsx! { div { class: "emanate-out", style: "animation-delay: {delay}s;" } }
                    }
                }
                // Inward rings
                for i in 0..2 {
                    {
                        let delay = i as f64 * 5.0;
                        rsx! { div { class: "emanate-in", style: "animation-delay: {delay}s;" } }
                    }
                }

                svg {
                    view_box: "0 0 800 800",
                    style: "width: 100%; height: 100%; overflow: visible; filter: drop-shadow(0 0 100px rgba(0,0,0,0.95)); position: relative; z-index: 20;",

                    defs {
                        linearGradient { id: "goldGradient", x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                            stop { offset: "0%", stop_color: "#BF953F" }
                            stop { offset: "50%", stop_color: "#FCF6BA" }
                            stop { offset: "100%", stop_color: "#AA771C" }
                        }
                        radialGradient { id: "dialGradient", cx: "50%", cy: "50%", r: "50%",
                            stop { offset: "0%", stop_color: "#1a1a1a" }
                            stop { offset: "80%", stop_color: "#080808" }
                            stop { offset: "100%", stop_color: "#000" }
                        }
                        // Universal Safe-Zone Glow Filters (Prevent ViewBox Clipping)
                        filter { id: "innerGlow", filterUnits: "userSpaceOnUse", x: "0", y: "0", width: "800", height: "800",
                            feGaussianBlur { std_deviation: "20", _in: "SourceAlpha", result: "blur" }
                            feOffset { dx: "0", dy: "0" }
                            feComposite { _in: "SourceAlpha", in2: "blur", operator: "arithmetic", k2: "-1", k3: "1" }
                            feColorMatrix { type: "matrix", values: "0 0 0 0 0.83  0 0 0 0 0.68  0 0 0 0 0.21  0 0 0 0 0.6 0" }
                        }
                        filter { id: "luxuryGlow", filterUnits: "userSpaceOnUse", x: "-100", y: "-100", width: "1000", height: "1000",
                            feGaussianBlur { std_deviation: "22", result: "blur" }
                            feColorMatrix { type: "matrix", values: "1 0 0 0 0  0 1 0 0 0  0 0 1 0 0  0 0 0 1.5 0" }
                            feComposite { _in: "SourceGraphic", in2: "blur", operator: "over" }
                        }
                        filter { id: "handShadow",
                            feDropShadow { dx: "5", dy: "5", std_deviation: "4", flood_opacity: "0.8" }
                        }
                    }

                    // Watch Face Base with Deep Inner Glow (Non-Clipped)
                    circle { cx: "400", cy: "400", r: "248", fill: "none", stroke: "url(#goldGradient)", stroke_width: "3" }
                    circle { cx: "400", cy: "400", r: "245", fill: "url(#dialGradient)", stroke: "rgba(255,255,255,0.05)", stroke_width: "1" }
                    circle { cx: "400", cy: "400", r: "245", fill: "none", filter: "url(#innerGlow)" }

                    // Tick Marks (Temporal Wake Implementation)
                    for i in 0..60 {
                        {
                            let angle = (i as f64 * 6.0).to_radians();
                            let is_five = i % 5 == 0;
                            let is_primary = i == 0 || i == 15 || i == 30 || i == 45; 
                            let has_passed = (i as u32) <= t.minute();
                            
                            let r_in = if is_primary { 205.0 } else if is_five { 218.0 } else { 235.0 };
                            let x1 = 400.0 + r_in * angle.sin();
                            let y1 = 400.0 - r_in * angle.cos();
                            let x2 = 400.0 + 242.0 * angle.sin();
                            let y2 = 400.0 - 242.0 * angle.cos();
                            
                            let stroke_color = if has_passed { "url(#goldGradient)" } else if is_five { "#666" } else { "#222" };
                            let stroke_width = if is_primary { "6" } else if is_five { "2.5" } else { "1" };
                            let opacity = if has_passed { "1.0" } else if is_five { "0.4" } else { "0.15" };

                            rsx! {
                                line { 
                                    x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}", 
                                    stroke: "{stroke_color}", 
                                    stroke_width: "{stroke_width}",
                                    opacity: "{opacity}",
                                    stroke_linecap: "round"
                                }
                            }
                        }
                    }

                    // Minute Numerals (Subtle Guidance)
// Minute Numerals (Corrected Temporal Guidance)
                    for i in 0..12 {
                        {
                            // 'm' represents the minute value (0, 5, 10... 55)
                            let m = i * 5; 

                            // IMPORTANT: The angle must be based on the minute value 'm'
                            // 360 degrees / 60 minutes = 6.0 degrees per minute
                            let angle = (m as f64 * 6.0).to_radians();

                            // Positioning the numbers on a radius of 225
                            let x = 400.0 + 225.0 * angle.sin();
                            let y = 400.0 - 225.0 * angle.cos();

                            // Highlight logic for the current 5-minute block
                            let is_current_five = (t.minute() / 5) == i as u32;
                            let opacity = if is_current_five { "0.6" } else { "0.15" };

                            rsx! {
                                text {
                                    x: "{x}", y: "{y}", 
                                    text_anchor: "middle", 
                                    alignment_baseline: "middle",
                                    fill: "rgba(212, 175, 55, {opacity})",
                                    font_size: "12", 
                                    font_family: "Montserrat", 
                                    font_weight: "600",
                                    style: "transition: all 0.5s ease;",
                                    // Ensure double digits (00, 05, 10...)
                                    "{m:02}"
                                }
                            }
                        }
                    }

                    // Full Circular 360° Celestial Aura (Omni-Glow Foundation)
                    circle { 
                        cx: "400", cy: "400", r: "241", fill: "none", 
                        stroke: "url(#goldGradient)", stroke_width: "8",
                        opacity: "0.9",
                        filter: "url(#luxuryGlow)"
                    }
                    // Animated Breathing Ring (Universal Pulse)
                    circle { 
                        cx: "400", cy: "400", r: "241", fill: "none", 
                        class: "rim-glow-breathe",
                        stroke: "rgba(212, 175, 55, 0.5)", stroke_width: "16",
                        filter: "url(#luxuryGlow)"
                    }

                    // Orbital Minute Indicator Path
                    circle { 
                        cx: "400", cy: "400", r: "230", fill: "none", 
                        stroke: "rgba(212, 175, 55, 0.05)", stroke_width: "1" 
                    }
                    
                    // The "Spirit Dot" Minute Indicator
                    circle { 
                        cx: "{minute_dot_x}", cy: "{minute_dot_y}", r: "4", 
                        fill: "#FCF6BA", 
                        filter: "url(#luxuryGlow)",
                        style: "transition: all 0.1s ease-out;"
                    }

                    // Daily Progress Highlight Arc (Adds extra intensity to passed time)
                    circle { 
                        cx: "400", cy: "400", r: "241", fill: "none", 
                        stroke: "url(#goldGradient)", stroke_width: "4",
                        stroke_dasharray: "{stroke_dasharray} 1514",
                        stroke_linecap: "round",
                        style: "transform: rotate(-90deg); transform-origin: 400px 400px; opacity: 1.0; filter: url(#luxuryGlow);"
                    }

                    // Markers
                    for h in 0..12 {
                        {
                            let angle = (h as f64 * 30.0).to_radians();
                            let x = 400.0 + 195.0 * angle.sin();
                            let y = 400.0 - 195.0 * angle.cos();
                            
                            let is_active = (t.hour() % 12) == h;
                            let date_key = format!("{}-{}", t.format("%Y-%m-%d"), h);
                            let has_note = notes.read().contains_key(&date_key);
                            let is_quadrant = h % 3 == 0;
                            
                            let marker_radius = if is_active { "12" } else if is_quadrant { "8" } else { "4" };
                            let marker_fill = if has_note { "#FFD700" } else if is_active || is_quadrant { "#FCF6BA" } else { "#333" };
                            let text_fill = if is_active || is_quadrant { "#FCF6BA" } else { "#444" };
                            let text_size = if is_active { "26" } else if is_quadrant { "18" } else { "14" };
                            let display_h = if h == 0 { 12 } else { h };
                            
                            rsx! {
                                g {
                                    onclick: move |_| selected_hour.set(Some(h as u32)),
                                    style: "cursor: pointer;",
                                    circle {
                                        cx: "{x}", cy: "{y}", r: "{marker_radius}",
                                        fill: "{marker_fill}",
                                        stroke: "url(#goldGradient)", stroke_width: "1.5",
                                        style: "transition: all 0.4s ease;"
                                    }
                                    text {
                                        x: "{x}", y: "{y}", dy: "-30", text_anchor: "middle",
                                        fill: "{text_fill}",
                                        font_size: "{text_size}",
                                        font_family: "Cinzel",
                                        font_weight: if is_active || is_quadrant { "700" } else { "200" },
                                        style: "transition: all 0.4s ease;",
                                        "{display_h}"
                                    }
                                }
                            }
                        }
                    }

                    // Hands Layer
                    g {
                        style: "transform: rotate({hour_deg}deg); transform-origin: 400px 400px; transition: transform 0.1s ease-out;",
                        line { x1: "400", y1: "400", x2: "400", y2: "295", stroke: "url(#goldGradient)", stroke_width: "14", stroke_linecap: "round", filter: "url(#handShadow)" }
                    }
                    g {
                        style: "transform: rotate({minute_deg}deg); transform-origin: 400px 400px; transition: transform 0.1s ease-out;",
                        line { x1: "400", y1: "400", x2: "400", y2: "215", stroke: "#FCF6BA", stroke_width: "6", stroke_linecap: "round", filter: "url(#handShadow)" }
                    }
                    g {
                        style: "transform: rotate({second_deg}deg); transform-origin: 400px 400px;",
                        line { x1: "400", y1: "430", x2: "400", y2: "190", stroke: "#AA771C", stroke_width: "2" }
                        circle { cx: "400", cy: "190", r: "6", fill: "#FCF6BA", filter: "url(#luxuryGlow)" }
                    }
                    // Refined Circular Hub with Pulsing Center Pin
                    circle { 
                        cx: "400", cy: "400", r: "22", 
                        fill: "url(#goldGradient)", 
                        filter: "url(#luxuryGlow)",
                        style: "transform-origin: 400px 400px; animation: pulse-hub 6s infinite ease-in-out;"
                    }
                    circle { cx: "400", cy: "400", r: "5", fill: "#FCF6BA", filter: "url(#luxuryGlow)" }
                }
            }

            // 3. UI Overlays (Absolute Corners for Center Focus)
            
            // Header: Branding
            div {
                style: "position: absolute; top: 7%; text-align: center; width: 100%; z-index: 50;",
                h1 { 
                    class: "gold-text", 
                    style: "font-size: 1.9rem; letter-spacing: 22px; margin: 0; font-weight: 900; line-height: 1.2; text-transform: uppercase;", 
                    "CHRONOS PLANTACERIUM" 
                }
                div { 
                    style: "color: #D4AF37; letter-spacing: 11px; font-size: 0.7rem; margin-top: 9px; opacity: 0.7; font-family: 'Cinzel', serif; font-weight: 700;", 
                    "AETERNUM PRECISION ARCHIVE" 
                }
            }

            // Bottom Left: Units of Presence
            div {
                style: "position: absolute; bottom: 6%; left: 6%; display: flex; flex-direction: column; background: rgba(5,5,5,0.7); padding: 25px 45px; border: 1px solid rgba(212,175,55,0.15); border-radius: 24px; backdrop-filter: blur(25px); z-index: 50;",
                div { style: "font-size: 0.75rem; color: #555; letter-spacing: 5px; text-transform: uppercase; margin-bottom: 5px;", "Units of Experience" }
                div { class: "gold-text", style: "font-size: 2.5rem; font-weight: 900; font-family: 'Cinzel', serif;", "{experience_points}" }
            }

            // Bottom Right: Secure State Button
            div {
                style: "position: absolute; bottom: 6%; right: 6%; z-index: 50;",
                button { 
                    class: "luxury-btn", 
                    style: "padding: 15px 45px; font-size: 0.9rem; backdrop-filter: blur(10px); min-width: 280px;",
                    onclick: on_save, 
                    "Secure State" 
                }
            }

            // Footer: Versioning (Centered deeply)
            div {
                style: "position: absolute; bottom: 3%; width: 100%; text-align: center; opacity: 0.35; letter-spacing: 8px; font-size: 0.65rem; font-family: 'Cinzel', serif; pointer-events: none;",
                "LIFE BANK EXPERIENCE V1 • TIME ANCHOR SYSTEM"
            }

            // Save Confirmation Notification
            if save_signal() {
                div { class: "save-status", "TIME VAULT SECURED" }
            }

            // 4. Modal: Temporal Observation Vault
            if let Some(h) = selected_hour() {
                {
                    let date_key = format!("{}-{}", Local::now().format("%Y-%m-%d"), h);
                    let note_content = notes.read().get(&date_key).map(|n| n.content.clone()).unwrap_or_default();
                    
                    rsx! {
                        div {
                            style: "position: absolute; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0,0,0,0.96); backdrop-filter: blur(40px); display: flex; justify-content: center; align-items: center; z-index: 1000;",
                            onclick: move |_| selected_hour.set(None),
                            div {
                                style: "width: 850px; height: 85vh; background: #080808; border: 1px solid #1a1a1a; padding: 70px; border-radius: 2px; box-shadow: 0 60px 120px rgba(0,0,0,1); display: flex; flex-direction: column; gap: 40px;",
                                onclick: move |e| e.stop_propagation(),
                                
                                header {
                                    style: "display: flex; justify-content: space-between; align-items: center;",
                                    div {
                                        h2 { class: "gold-text", style: "font-family: Cinzel; margin: 0; font-size: 3.5rem; letter-spacing: 20px; font-weight: 900;", "HOUR {display_modal_h}" }
                                        div { style: "font-size: 0.9rem; color: #444; letter-spacing: 12px; text-transform: uppercase; margin-top: 10px;", "Temporal Observation Node" }
                                    }
                                    button {
                                        class: "luxury-btn",
                                        style: "padding: 12px 30px; font-size: 0.8rem;",
                                        onclick: move |_| selected_hour.set(None),
                                        "Lock Node"
                                    }
                                }

                                // Input Section
                                textarea {
                                    style: "width: 100%; height: 220px; background: #000; color: #FCF6BA; border: 1px solid #1a1a1a; padding: 35px; font-family: 'Montserrat', sans-serif; font-size: 1.2rem; outline: none; line-height: 1.8; resize: none;",
                                    value: "{note_content}",
                                    placeholder: "Commit the essence of this temporal anchor to memory...",
                                    oninput: move |e| {
                                        let mut current_notes = notes.write();
                                        let date_key = format!("{}-{}", Local::now().format("%Y-%m-%d"), h);
                                        current_notes.insert(date_key, TimeNote { content: e.value(), is_locked: false });
                                    }
                                }

                                // Preview Section (Stacked Below Input)
                                div {
                                    style: "flex: 1; overflow-y: auto; padding: 50px; background: rgba(10,10,10,0.5); border: 1px solid rgba(212,175,55,0.08);",
                                    div { 
                                        class: "markdown-body", 
                                        dangerous_inner_html: "{render_markdown(&note_content)}" 
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Helper to render Markdown to HTML
fn render_markdown(text: &str) -> String {
    let parser = pulldown_cmark::Parser::new(text);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
