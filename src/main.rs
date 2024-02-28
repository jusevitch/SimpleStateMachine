use std::io;

#[derive(Debug)]
enum State {
    INIT,
    PREFLIGHT,
    CALIBRATING,
    ARMED,
    FAILSAFE,
    ERROR
}

#[derive(Debug)]
enum Action {
    Initialized,
    CalibrationComplete,
    CalibrationFailed,
    ArmReqAndCalibrateOnArm,
    ArmRequested,
    DisarmRequested,
    DisarmRequestedAndError,
    RCLost,
    RCRegained,
    Error,
    ErrorsCleared,
    Invalid,
}

fn parse_action_input(input: &str) -> Result<Action, &str> {
    match input.trim() {
        "Initialized" => Ok(Action::Initialized),
        "CalibrationComplete" => Ok(Action::CalibrationComplete),
        "CalibrationFailed" => Ok(Action::CalibrationFailed),
        "ArmReqAndCalibrateOnArm" => Ok(Action::ArmReqAndCalibrateOnArm),
        "ArmRequested" => Ok(Action::ArmRequested),
        "DisarmRequested" => Ok(Action::DisarmRequested),
        "DisarmRequestedAndError" => Ok(Action::DisarmRequestedAndError),
        "RCLost" => Ok(Action::RCLost),
        "RCRegained" => Ok(Action:: RCRegained),
        "Error" => Ok(Action::Error),
        "ErrorsCleared" => Ok(Action::ErrorsCleared),
        _ => Err("Invalid input."),
    }
}


fn main() {
    let mut current_state: State = State::INIT;

    loop {
        println!("Current state: {:?}", current_state);
        println!("Enter next action:");

        let mut command_str = String::new();
        io::stdin()
            .read_line(&mut command_str)
            .expect("Failed to read line.");

        let next_action = match parse_action_input(&command_str) {
            Ok(action) => action,
            Err(_) => Action::Invalid,
        };

        let next_state_result: Result<State, &str> = match (&current_state, next_action) {
            (State::INIT, Action::Initialized) => Ok(State::PREFLIGHT),
            (State::PREFLIGHT, Action::ArmReqAndCalibrateOnArm) => Ok(State::CALIBRATING),
            (State::PREFLIGHT, Action::ArmRequested) => Ok(State::ARMED),
            (State::PREFLIGHT, Action::Error) => Ok(State::ERROR),
            (State::CALIBRATING, Action::CalibrationFailed) => Ok(State::PREFLIGHT),
            (State::CALIBRATING, Action::CalibrationComplete) => Ok(State::ARMED),
            (State::CALIBRATING, Action::Error) => Ok(State::ERROR),
            (State::ARMED, Action::DisarmRequested) => Ok(State::PREFLIGHT),
            (State::ARMED, Action::DisarmRequestedAndError) => Ok(State::ERROR),
            (State::ARMED, Action::RCLost) => Ok(State::FAILSAFE),
            (State::FAILSAFE, Action::RCRegained) => Ok(State::ARMED),
            (State::FAILSAFE, Action::DisarmRequested) => Ok(State::ERROR),
            (State::ERROR, Action::ErrorsCleared) => Ok(State::PREFLIGHT),
            (_,_) => Err("Invalid action!")
        };

        match next_state_result {
            Ok(state) => current_state = state,
            Err(msg) => println!("{:?}", msg)
        }

    }
}
