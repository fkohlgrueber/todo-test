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
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    ToggleEdit(usize),
    Toggle(usize),
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
            edit_value: "".into(),
        };
        Model {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    completed: false,
                    editing: false,
                };
                self.state.entries.push(entry);
                self.state.value = "".to_string();
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.clone();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.toggle_edit(idx);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
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
                    <header class="header">
                        <h1>{ "todos" }</h1>
                    </header>
                    <section class="main">
                        <ul class="todo-list">
                            { for self.state.entries.iter().enumerate().map(|e| self.view_entry(e)) }
                        </ul>
                    </section>
                    { self.view_input() }
                </section>
            </div>
        }
    }
}

impl Model {
    fn view_input(&self) -> Html {
        html! {
            <input class="new-todo"
                   placeholder="What needs to be done?"
                   value=&self.state.value
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                   onkeypress=self.link.callback(|e: KeyboardEvent| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   }) />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = "todo".to_string();
        if entry.editing {
            class.push_str(" editing");
        }
        if entry.completed {
            class.push_str(" completed");
        }
        html! {
            <li class=class>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked=entry.completed
                        onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                    <label ondoubleclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    <button class="destroy" onclick=self.link.callback(move |_| Msg::Remove(idx)) />
                </div>
                { self.view_entry_edit_input((idx, &entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input class="edit"
                       type="text"
                       value=&entry.description
                       oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value))
                       onblur=self.link.callback(move |_| Msg::Edit(idx))
                       onkeypress=self.link.callback(move |e: KeyboardEvent| {
                          if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                       }) />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

impl State {
    fn toggle(&mut self, idx: usize) {
        let mut entries = self
            .entries
            .iter_mut()
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let mut entries = self
            .entries
            .iter_mut()
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let mut entries = self
            .entries
            .iter_mut()
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.editing = !entry.editing;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let entries = self
                .entries
                .iter()
                .enumerate()
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}
