use nalgebra::{Vector2, Vector3};
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::input::{KeyboardInputData, MouseMoveData, MouseInputData, JoystickInputData, ButtonState};
use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{ptr_as_ref, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor::ActorController;
use crate::game_module::game_constants::{
    CAMERA_DISTANCE_MIN,
    CAMERA_DISTANCE_MAX,
    CAMERA_DISTANCE_SPEED,
    CAMERA_VERTICAL_OFFSET
};
use crate::game_module::game_client::GameClient;
use crate::game_module::game_ui::GameUIManager;


#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameViewMode {
    SideViewMode,
    Count
}

pub struct GameController {
    pub _game_client: *const GameClient,
    pub _game_ui_manager: *const GameUIManager,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _target_position: Vector3<f32>,
    pub _target_direction: Vector3<f32>,
    pub _relative_target_position: Vector3<f32>,
    pub _game_view_mode: GameViewMode,
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        let default_camera_distance = (CAMERA_DISTANCE_MIN + CAMERA_DISTANCE_MAX) * 0.5;
        Box::new(GameController {
            _game_client: std::ptr::null(),
            _game_ui_manager: std::ptr::null(),
            _camera_distance: default_camera_distance,
            _camera_goal_distance: default_camera_distance,
            _target_position: Vector3::zeros(),
            _target_direction: Vector3::zeros(),
            _relative_target_position: Vector3::zeros(),
            _game_view_mode: GameViewMode::SideViewMode,
        })
    }

    pub fn initialize_game_controller(&mut self, game_client: &GameClient) {
        self._game_client = game_client;
        self._game_ui_manager = game_client._game_ui_manager.as_ref();
        self.change_view_mode(GameViewMode::SideViewMode);
    }
    pub fn get_game_client(&self) -> &GameClient { ptr_as_ref(self._game_client) }
    pub fn get_game_client_mut(&self) -> &mut GameClient { ptr_as_mut(self._game_client) }
    pub fn get_game_ui_manager(&self) -> &GameUIManager { ptr_as_ref(self._game_ui_manager) }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { ptr_as_mut(self._game_ui_manager) }
    pub fn get_main_camera(&self) -> &CameraObjectData {
        self.get_game_client().get_project_scene_manager().get_main_camera()
    }
    pub fn get_main_camera_mut(&self) -> &mut CameraObjectData {
        self.get_game_client().get_project_scene_manager().get_main_camera_mut()
    }
    pub fn is_view_mode(&self, target_view_mode: GameViewMode) -> bool {
        if target_view_mode == self._game_view_mode { true } else { false }
    }
    pub fn change_view_mode(&mut self, view_mode: GameViewMode) {
        let is_side_view_mode = GameViewMode::SideViewMode == view_mode;
        self.get_game_ui_manager_mut().show_selection_area(false == is_side_view_mode);
        self.get_game_ui_manager_mut().set_crosshair_tracking_mouse(is_side_view_mode);
        self._game_view_mode = view_mode;
    }
    pub fn toggle_view_mode(&mut self) {
        let next_view_mode = (self._game_view_mode as i32 + 1) % GameViewMode::Count as i32;
        self.change_view_mode(unsafe { std::mem::transmute(next_view_mode) });
    }
    pub fn get_camera_distance_ratio(&self) -> f32 {
        (self._camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN)
    }
    pub fn update_camera_distance(&mut self, distance: f32) {
        self._camera_goal_distance += distance;
        if self._camera_goal_distance < CAMERA_DISTANCE_MIN {
            self._camera_goal_distance = CAMERA_DISTANCE_MIN;
        } else if CAMERA_DISTANCE_MAX < self._camera_goal_distance {
            self._camera_goal_distance = CAMERA_DISTANCE_MAX;
        }
    }
    pub fn update_target_position(&mut self, _project_scene_manager: &ProjectSceneManager, _main_camera: &CameraObjectData, _mouse_pos: &Vector2<i32>) {
        // todo
        // let relative_pos = main_camera.convert_screen_to_relative_world(mouse_pos);
        // self._target_direction = self._target_position.normalize();
        // self._relative_target_position = self._target_position - main_camera._transform_object.get_position();
    }

    pub fn update_event_for_side_view_mode(
        &mut self,
        _time_data: &TimeData,
        joystick_input_data: &JoystickInputData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _main_camera: &mut CameraObjectData,
        player_actor: &mut ActorController
    ) {
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let hold_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let hold_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let hold_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let hold_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

        if btn_left || ButtonState::Pressed == joystick_input_data._btn_a {
            player_actor.set_command_actor_fire();
        }

        if modifier_keys_shift {
            player_actor.get_ship_mut().get_controller_mut().boost_on();
        }

        if hold_key_a || joystick_input_data._btn_left == ButtonState::Hold || joystick_input_data._stick_left_direction.x < 0 {
            player_actor.get_ship_mut().get_controller_mut().set_yaw(-std::f32::consts::PI * 0.5);
            player_actor.set_command_actor_walk();
        }
        else if hold_key_d || joystick_input_data._btn_right == ButtonState::Hold || 0 < joystick_input_data._stick_left_direction.x {
            player_actor.get_ship_mut().get_controller_mut().set_yaw(std::f32::consts::PI * 0.5);
            player_actor.set_command_actor_walk();
        }

        if hold_key_w || joystick_input_data._btn_up == ButtonState::Hold || joystick_input_data._btn_left_bumper == ButtonState::Hold || joystick_input_data._stick_left_direction.y < 0 {
            player_actor.get_ship_mut().get_controller_mut().acceleration_vertical(1.0);
        }
        else if hold_key_s || joystick_input_data._btn_down == ButtonState::Hold || joystick_input_data._btn_right_bumper == ButtonState::Hold || 0 < joystick_input_data._stick_left_direction.y {
            player_actor.get_ship_mut().get_controller_mut().acceleration_vertical(-1.0);
        }
    }

    pub fn update_camera(&mut self, delta_time: f32) {
        if self._camera_goal_distance != self._camera_distance {
            self._camera_distance = math::lerp(self._camera_distance, self._camera_goal_distance, 1.0f32.min(delta_time * CAMERA_DISTANCE_SPEED));
        }

        let player_actor = self.get_game_client().get_actor_manager().get_player_actor();
        let main_camera = self.get_main_camera_mut();
        let player_transform = player_actor.get_transform();

        if GameViewMode::SideViewMode == self._game_view_mode {
            main_camera._transform_object.set_yaw(std::f32::consts::PI);
            let mut camera_pos = player_transform.get_position().clone_owned();
            camera_pos.y += CAMERA_VERTICAL_OFFSET;
            camera_pos.z -= self._camera_distance;
            main_camera._transform_object.set_position(&camera_pos);
            // main_camera._transform_object.update_transform_object();
        } else {
            assert!(false, "Not implemented.");
        }
    }

    pub fn update_game_controller(&mut self, delta_time: f32) {
        self.update_camera(delta_time);
    }
}