use cgmath::InnerSpace;
use winit::event::{WindowEvent, VirtualKeyCode, KeyboardInput, ElementState};

use crate::camera::{Camera, self};

pub struct CameraController {
    speed: f32,
    left_pressed: bool,
    right_pressed: bool,
    forward_pressed: bool,
    backward_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            left_pressed: false,
            right_pressed: false,
            forward_pressed: false,
            backward_pressed: false,
        }
    }

    pub fn handle_input(&mut self, input: &WindowEvent) -> bool {
        let WindowEvent::KeyboardInput { input: KeyboardInput {virtual_keycode: Some(keycode), state: pressed, ..}, is_synthetic, .. } = 
             input else {
            return false;
        };
        let key = match keycode {
            VirtualKeyCode::H => &mut self.left_pressed,
            VirtualKeyCode::L => &mut self.right_pressed,
            VirtualKeyCode::J => &mut self.backward_pressed,
            VirtualKeyCode::K => &mut self.forward_pressed,
            _ => return false
        };

        if *key && *pressed == ElementState::Released {
            *key = false;
        } else if !*key && *pressed == ElementState::Pressed {
            *key = true;
        }
        true
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let (normal, magnitude)= {
          let forward = (camera.target - camera.eye);
            (forward.normalize(), forward.magnitude())        
        };

        if self.forward_pressed {
            camera.eye += self.speed * normal;
            camera.target += self.speed * normal;
        }
        if self.backward_pressed {
            camera.eye -= self.speed * normal;
            camera.target -= self.speed * normal;
        }
        if self.left_pressed {
            camera.eye -= (self.speed * normal).cross(camera.up);
            camera.target -= (self.speed * normal).cross(camera.up);
        }
        if self.right_pressed {
            camera.eye += (self.speed * normal).cross(camera.up);
            camera.target += (self.speed * normal).cross(camera.up);
        }
    }
}
