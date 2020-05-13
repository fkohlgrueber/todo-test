use serde_derive::{Deserialize, Serialize};
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

const KEY: &str = "yew.todomvc.self";

pub struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
}

pub enum Msg {
    
    Add,
    UpdateInput(String),
    Update(usize, String),
    Toggle(usize),
    Remove(usize),
    Nope,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let entries = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            value: "".into(),
        };
        Model {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Remove(idx) => {
                self.state.entries.remove(idx);
            }
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    completed: false,
                };
                self.state.entries.push(entry);
                self.state.value = "".to_string();
            }
            Msg::UpdateInput(val) => {
                self.state.value = val;
            }
            Msg::Update(idx, val) => {
                self.state.entries[idx].description = val;
            }
            Msg::Toggle(idx) => {
                let entry = self.state.entries.get_mut(idx).unwrap();
                entry.completed = !entry.completed;
            }
            Msg::Nope => {}
        }
        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    // header
                    <header class="header">
                        <h1>{ "ToDo" }</h1>
                    </header>
                    
                    
                    // todo items
                    <div class="entries">
                        { for self.state.entries.iter().enumerate().map(|e| self.view_entry(e)) }
                    </div>

                    // input line
                    <div class="entry input-entry">
                        // the checkbox
                        <span
                            class="checkbox"
                        />
                        // description
                        <input 
                            placeholder="Add item"
                            class="desc"
                            type="text"
                            value=&self.state.value
                            oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
                            onkeypress=self.link.callback(|e: KeyboardEvent| {
                                if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                            }) />
                        <span class="remove" >{"\u{00D7}"}</span>
                    </div>
                </section>
            </div>
        }
    }
}

impl Model {
    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut chb_class = "checkbox ".to_string();
        if entry.completed { 
            chb_class.push_str("checked ") 
        }
        let mut entry_class = "entry ".to_string();
        if entry.description.is_empty() {
            entry_class.push_str("empty");
        }
        html! {
            <div class=entry_class>
                // the checkbox
                <span
                    class=chb_class
                    onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                // description
                <input 
                    class="desc"
                    type="text"
                    value=&entry.description
                    oninput=self.link.callback(move |e: InputData| Msg::Update(idx, e.value))
                    /*onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                    })*/ />
                <span class="remove" onclick=self.link.callback(move |_| Msg::Remove(idx)) >{"\u{00D7}"}</span>
            </div>
        }
    }
}
