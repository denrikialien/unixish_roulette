fn main() {
    let mut table = Table {
        revolver: Revolver {
            chambers: vec![
                Chamber::Empty,
                Chamber::Bullet,
                Chamber::Empty,
                Chamber::Empty,
                Chamber::Empty,
                Chamber::Empty,
            ],
        },
        hands: vec![
            Hand {
                id: PlayerId(0),
                prev_action: None,
            },
            Hand {
                id: PlayerId(1),
                prev_action: None,
            },
        ],
    };

    let result = loop {
        match perform_action(table, Action::Trigger) {
            State::Playing(t) => table = dbg!(t),
            t => break t,
        }
    };

    println!("{:?}", result);
}

#[derive(Debug, PartialEq)]
struct Table {
    revolver: Revolver,
    hands: Vec<Hand>,
}

#[derive(Debug, PartialEq)]
enum State {
    Playing(Table),
    SomeoneDied(PlayerId),
    SomeoneWon(PlayerId),
    EverybodyFolded,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PlayerId(usize);

#[derive(Debug, PartialEq)]
struct Hand {
    id: PlayerId,
    prev_action: Option<Action>,
}

#[derive(Debug, PartialEq)]
struct Revolver {
    chambers: Vec<Chamber>,
}

#[derive(Debug, PartialEq)]
enum Chamber {
    Empty,
    Bullet,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Action {
    Fold,
    Slide,
    Trigger,
}

#[derive(Debug, PartialEq)]
enum Shot {
    Fired,
    Blank,
}

fn current_hand(table: &Table) -> &Hand {
    assert!(!table.hands.is_empty());
    table.hands.last().unwrap()
}

fn current_hand_mut(table: &mut Table) -> &mut Hand {
    assert!(!table.hands.is_empty());
    table.hands.last_mut().unwrap()
}

fn fold_current_hand(table: &mut Table) -> Hand {
    assert!(!table.hands.is_empty());
    table.hands.pop().unwrap()
}

fn perform_action(table: Table, action: Action) -> State {
    match action {
        Action::Fold => perform_fold(table),
        Action::Slide => perform_slide(table),
        Action::Trigger => perform_trigger(table),
    }
}

fn perform_trigger(mut table: Table) -> State {
    match pull_trigger(&mut table.revolver) {
        Shot::Fired => State::SomeoneDied(fold_current_hand(&mut table).id),
        Shot::Blank => match table.revolver.chambers.is_empty() {
            true => State::SomeoneWon(fold_current_hand(&mut table).id),
            false => slide_revolver(table, Action::Trigger),
        },
    }
}

fn perform_fold(mut table: Table) -> State {
    fold_current_hand(&mut table);
    match table.hands.is_empty() {
        true => State::EverybodyFolded,
        false => slide_revolver(table, Action::Fold),
    }
}

fn perform_slide(table: Table) -> State {
    match current_hand(&table).prev_action {
        Some(Action::Slide) => perform_action(table, Action::Fold),
        _ => slide_revolver(table, Action::Slide),
    }
}

fn slide_revolver(mut table: Table, action_took: Action) -> State {
    current_hand_mut(&mut table).prev_action = Some(action_took);
    table.hands.rotate_right(1);
    State::Playing(table)
}

fn pull_trigger(revolver: &mut Revolver) -> Shot {
    match revolver.chambers.pop() {
        None | Some(Chamber::Empty) => Shot::Blank,
        Some(Chamber::Bullet) => Shot::Fired,
    }
}
