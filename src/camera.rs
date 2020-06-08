use quicksilver::prelude::*;

pub struct Camera {
    //position: Position<u32>,
    //viewport_size: Vector,
    pub z_position: u32,
    pub viewport: Rectangle,
    pub zoom_factor: f32,
    pub max_x: u32,
    pub max_y: u32,
    pub max_z: u32,
    zoom_interval: f32,
    ref_camera: Rectangle,
}

impl Camera {

    pub fn new(x_position: u32, y_position: u32, z_position: u32,
           max_x: u32, max_y: u32, max_z: u32,
           viewport_size: impl Into<Vector>) -> Camera {
        let reference_camera = Rectangle::new((x_position, y_position), viewport_size);
        Camera {
            z_position: z_position,
            ref_camera: reference_camera,
            viewport: reference_camera.clone(),
            max_x: max_x,
            max_y: max_y,
            max_z: max_z,
            zoom_factor: 1.0,
            zoom_interval: 0.1,
        }
    }

    pub fn move_left(&mut self) {
        let delta = -1.0 / self.zoom_factor;
        if self.viewport.x() + delta >= 0.0 {
            self.ref_camera = self.ref_camera
                .translate((delta , 0));
            self.rescale(); 
        }
    }

    pub fn move_right(&mut self) {
        let delta = 1.0 / self.zoom_factor;
        if self.viewport.x() + delta < self.max_x as f32 {
            self.ref_camera = self.ref_camera
                .translate((delta, 0));
            self.rescale(); 
        }
    }

    pub fn move_up(&mut self) {
        let delta = -1.0 / self.zoom_factor;
        if self.viewport.y() + delta >= 0.0 {
            self.ref_camera = self.ref_camera
                .translate((0, delta));
            self.rescale(); 
        }
    }

    pub fn move_down(&mut self) {
        let delta = 1.0 / self.zoom_factor;
        if self.viewport.y() + delta < self.max_y as f32 {
            self.ref_camera = self.ref_camera
                .translate((0, delta));
            self.rescale(); 
        }
    }

    pub fn elevate(&mut self) {
        if self.z_position > 0 {
            self.z_position -= 1;
        }
    }

    pub fn lower(&mut self) {
        if self.z_position < self.max_z {
            self.z_position += 1;
        }
    }

    pub fn go_to(&mut self, x: f32, y: f32, z: u32) {
        if x <= self.max_x as f32 && 
           y <= self.max_y as f32 && 
           z <= self.max_z {
           self.ref_camera = Rectangle::new(
               (x, y), self.ref_camera.size()
           );    
           self.z_position = z;
        }
    }
    
    pub fn zoom_in(&mut self) {
        self.zoom_factor += self.zoom_interval;
        self.rescale();
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_factor > 0.2 {
            self.zoom_factor -= self.zoom_interval;
            self.rescale(); 
        }
    }

    fn rescale(&mut self) {
        let scaled_width = self.ref_camera.width() / self.zoom_factor;
        let scaled_height = self.ref_camera.height() / self.zoom_factor;
        let center = self.ref_camera.center();
        self.viewport = Rectangle::new_sized((scaled_width, scaled_height))
            .with_center((center.x, center.y));
    }
}

 
