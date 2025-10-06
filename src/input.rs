use bevy::prelude::*;

use crate::scheduling::GameSchedule;

#[derive(PartialEq)]
pub enum InputEventType {
  Pressed,
  Released,
}

#[derive(PartialEq, Clone)]
pub enum InputEventAction {
  Shoot,
  Shield,
  Pause,
}

#[derive(Resource)]
struct KeyBindings {
  up_keys: Vec<KeyCode>,
  down_keys: Vec<KeyCode>,
  left_keys: Vec<KeyCode>,
  right_keys: Vec<KeyCode>,
  commands: Vec<KeyCommand>,
}

struct KeyCommand {
  action: InputEventAction,
  keys: Vec<KeyCode>,
}
impl KeyCommand {
  fn new(action: InputEventAction, keys: Vec<KeyCode>) -> Self {
    Self { action, keys }
  }
}

impl Default for KeyBindings {
  fn default() -> Self {
    Self {
      up_keys: vec![KeyCode::KeyW, KeyCode::ArrowUp],
      down_keys: vec![KeyCode::KeyS, KeyCode::ArrowDown],
      left_keys: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
      right_keys: vec![KeyCode::KeyD, KeyCode::ArrowRight],
      commands: vec![
        KeyCommand::new(
          InputEventAction::Shoot,
          vec![KeyCode::Space, KeyCode::ShiftRight],
        ),
        KeyCommand::new(
          InputEventAction::Shield,
          vec![KeyCode::ControlLeft, KeyCode::ControlRight],
        ),
        KeyCommand::new(
          InputEventAction::Pause,
          vec![KeyCode::Escape, KeyCode::Pause],
        ),
      ],
    }
  }
}

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<KeyBindings>()
      .add_message::<InputMovementMessage>()
      .init_resource::<KeyBindings>()
      .add_message::<InputTriggerMessage>()
      .add_systems(Startup, init_input_resources)
      .add_systems(Update, (read_keys, read_touch, read_gamepads).in_set(GameSchedule::ReadUserInput));
  }
}

#[derive(Message)]
pub struct InputMovementMessage {
  pub direction: Vec2,
}

impl InputMovementMessage {
  pub fn new(direction: Vec2) -> Self {
    Self { direction }
  }
}

#[derive(Message)]
pub struct InputTriggerMessage {
  pub action: InputEventAction,
  pub input_type: InputEventType,
}

impl InputTriggerMessage {
  pub fn new(action: InputEventAction, input_type: InputEventType) -> Self {
    Self { action, input_type }
  }
}
/*
#[derive(Resource)]
struct MouseResource {
  last: Vec2,
}
 */
#[derive(Resource)]
struct TouchResource {
  move_finger: Option<u64>,
  last: Vec2,
}

fn init_input_resources(mut commands: Commands) {
  //commands.insert_resource(MouseResource { last: Vec2::ZERO });
  commands.insert_resource(TouchResource {
    last: Vec2::ZERO,
    move_finger: None,
  });
}

fn read_gamepads(
  gamepads: Query<&Gamepad>,
  mut mw_movement_event: MessageWriter<InputMovementMessage>,
  mut mw_trigger_event: MessageWriter<InputTriggerMessage>,
  mut last_dir: Local<Vec2>,
) {
  for gamepad in &gamepads {
    if gamepad.just_pressed(GamepadButton::East) {
      mw_trigger_event.write(InputTriggerMessage::new(
        InputEventAction::Shield,
        InputEventType::Pressed,
      ));
    } else if gamepad.just_released(GamepadButton::East) {
      mw_trigger_event.write(InputTriggerMessage::new(
        InputEventAction::Shield,
        InputEventType::Released,
      ));
    }
    if gamepad.just_pressed(GamepadButton::South) {
      mw_trigger_event.write(InputTriggerMessage::new(
        InputEventAction::Shoot,
        InputEventType::Pressed,
      ));
    } else if gamepad.just_released(GamepadButton::South) {
      mw_trigger_event.write(InputTriggerMessage::new(
        InputEventAction::Shoot,
        InputEventType::Released,
      ));
    }
    let left_stick_x = (-1. * gamepad.get(GamepadAxis::LeftStickX).unwrap()).min(1.).max(-1.);
    let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap().min(1.).max(-1.);

    let mut dir: Vec2 = Vec2::new(-left_stick_x, left_stick_y);

    if gamepad.pressed(GamepadButton::DPadLeft){
      dir.x = -1.;
    }
    if gamepad.pressed(GamepadButton::DPadRight){
      dir.x = 1.;
    }
    if gamepad.pressed(GamepadButton::DPadUp){
      dir.y = 1.;
    }
    if gamepad.pressed(GamepadButton::DPadDown){
      dir.y = -1.;
    }



    
    if *last_dir != dir || dir.length_squared() > 0.1 {
      *last_dir = dir;
      mw_movement_event.write(InputMovementMessage::new(dir));
    }
  }
}

fn read_touch(
  touches: Res<Touches>,
  mut movement_writer: MessageWriter<InputMovementMessage>,
  mut trigger_writer: MessageWriter<InputTriggerMessage>,
  mut touch_tracker: ResMut<TouchResource>,

) {
  for touch in touches.iter_just_pressed() {
    //fisrt touch down is our move finger
    //info!("touch down: {:?}", touch.id());
    if touch_tracker.move_finger.is_none() {
      touch_tracker.move_finger = Some(touch.id());
      touch_tracker.last = touch.position();
    } else {
      //second is our shoot action
      
      trigger_writer.write(InputTriggerMessage::new(
        InputEventAction::Shoot,
        InputEventType::Pressed,
      ));
    }
  }

  for touch in touches.iter_just_released() {
    //release movement
    //info!("touch up: {:?}", touch.id());
    if touch_tracker.move_finger == Some(touch.id()) {
      touch_tracker.move_finger = None;
    } else {
      //or stop firing
      trigger_writer.write(InputTriggerMessage::new(
        InputEventAction::Shoot,
        InputEventType::Released,
      ));
    }
  }

  if let Some(finger) = touch_tracker.move_finger {
    let mut found = false;
    for touch in touches.iter() {
      //move finger movement tracking
      if finger == touch.id() {
        found = true;
        let diff = touch_tracker.last - touch.position();
        if diff.length_squared() > 0.5 {
          movement_writer.write(InputMovementMessage::new(diff * 2.));
        }
        touch_tracker.last = touch.position();
      }
    }
    if !found {
      touch_tracker.move_finger = None;
    }
  }
}
/*
fn read_mouse(
  buttons: Res<ButtonInput<MouseButton>>,
  window: Single<&Window, With<PrimaryWindow>>,
  mut ev_movement_event: EventWriter<InputMovementEvent>,
  mut ev_trigger_event: EventWriter<InputTriggerEvent>,
  mut mouse_location: ResMut<MouseResource>,
) {
  if buttons.just_pressed(MouseButton::Right) {
    ev_trigger_event.write(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Pressed,
    ));
  }
  if buttons.just_released(MouseButton::Right) {
    ev_trigger_event.write(InputTriggerEvent::new(
      InputEventAction::Shoot,
      InputEventType::Released,
    ));
  }

  if buttons.pressed(MouseButton::Left) {
    if let Some(pos) = window.cursor_position() {
      if buttons.just_pressed(MouseButton::Left) {
        mouse_location.last = pos;
      } else {
        let diff = mouse_location.last - pos;
        if diff.length_squared() > 0.5 {
          ev_movement_event.write(InputMovementEvent::new(diff * 2.));
        }
        mouse_location.last = pos;
      }
    }
  }
}
 */

fn read_keys(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut movement_writer: MessageWriter<InputMovementMessage>,
  mut trigger_writer: MessageWriter<InputTriggerMessage>,
  key_binds: Res<KeyBindings>,
  mut last_dir: Local<Vec2>,
) {
  let mut dir: Vec2 = Vec2::ZERO;

  if keyboard_input.any_pressed(key_binds.left_keys.clone()) {
    dir.x -= 1.;
  }
  if keyboard_input.any_pressed(key_binds.right_keys.clone()) {
    dir.x += 1.;
  }
  if keyboard_input.any_pressed(key_binds.up_keys.clone()) {
    dir.y += 1.;
  }
  if keyboard_input.any_pressed(key_binds.down_keys.clone()) {
    dir.y -= 1.;
  }

  if dir != *last_dir || dir != Vec2::ZERO {
    *last_dir = dir;
    movement_writer.write(InputMovementMessage::new(dir));
  }

  for command in key_binds.commands.iter() {
    if keyboard_input.any_just_pressed(command.keys.clone()) {
      trigger_writer.write(InputTriggerMessage::new(
        command.action.clone(),
        InputEventType::Pressed,
      ));
    }

    if keyboard_input.any_just_released(command.keys.clone()) {
      trigger_writer.write(InputTriggerMessage::new(
        command.action.clone(),
        InputEventType::Released,
      ));
    }
  }
}
