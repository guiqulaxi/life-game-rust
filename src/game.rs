extern crate rand;

use rand::Rng;
use piston_window::G2d;
use piston_window::types::Color;
use drawing::draw_block;
use piston_window::Context;
const LIFE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub struct Game{
    height:i32,
    width:i32,
    world:Vec<Vec<bool>>,
}
impl Game {
    //计算周围生命的数目
    fn sum_around(&self,row:i32,col:i32)->u32{
        let mut num:u32=0;
         for i in row-1..row+2 {
            for j in col-1..col+2 {
               
                if i<0||i>=self.height||j<0||j>=self.width ||(i==row&&j==col){
                    continue;
                }
                else {
                    if self.world[i as usize][j as usize]{
                        num+=1;
                    }
                    
                }
            }
         }
         
         num
    }
    pub fn next(& mut self){
        let mut new_world=vec![vec![false;self.width as usize];self.height as usize];
        for i in 0..self.height {
            for j in 0..self.width {
                match self.sum_around(i,j){
                    3=> new_world[i as usize ][j as usize]=true,//格子周围生命等于3，产生生命
                    2=> new_world[i as usize][j as usize]=self.world[i as usize][j as usize] ,//格子周围生命等于2，保持不变
                    _=> new_world[i as usize][j as usize]=false,//生命消失
                }
            }
         }
         self.world=new_world;
     }
    pub fn new( width :i32,height:i32)->Self{
        let mut world=vec![vec![false;width as usize];height as usize];
        //随机初始化
        let mut rng = rand::thread_rng();
          for i in 0..height {
            for j in 0..width {
                world[i as usize][j as usize]=rng.gen();
            }
        }
        
       Game{
           height:height,
           width:width,
           world:world,
       }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.world[i as usize][j as usize]{
                    draw_block(LIFE_COLOR, j, i, con, g);
                }
                
            }
        }     
    }
     pub fn update(&mut self, delta_time: f64) {
        self.next();
    }

}