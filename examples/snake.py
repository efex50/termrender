#!/usr/bin/env python3
"""
Termrender Snake Game Demo
A classic Snake game written entirely in Python, powered by the Termrender Rust Engine via PyO3!
"""

import time
import random
import sys

try:
    import termrender
except ImportError:
    print("Error: termrender is not installed correctly.")
    sys.exit(1)

class SnakeGame:
    def __init__(self, engine, width: int = 40, height: int = 20):
        self.engine = engine
        self.api = engine.get_api()
        self.width = width
        self.height = height
        
        # Colors
        color_green = termrender._termrender.GColor.green()
        color_red = termrender._termrender.GColor.red()
        color_wall = termrender._termrender.GColor.white()
        
        # Texts
        txt_head = termrender._termrender.TermPrint("O", color_red, None)
        txt_body = termrender._termrender.TermPrint("o", color_green, None)
        txt_food = termrender._termrender.TermPrint("*", color_red, None)
        txt_wall = termrender._termrender.TermPrint("#", color_wall, None)

        # Print Types
        pt_head = termrender._termrender.PrintType.new_string(0, 0, txt_head)
        pt_body = termrender._termrender.PrintType.new_string(0, 0, txt_body)
        pt_food = termrender._termrender.PrintType.new_string(0, 0, txt_food)
        pt_wall = termrender._termrender.PrintType.new_string(0, 0, txt_wall)
        
        # Textures
        self.tex_head = termrender._termrender.GameTexture([pt_head])
        self.tex_body = termrender._termrender.GameTexture([pt_body])
        self.tex_food = termrender._termrender.GameTexture([pt_food])
        self.tex_wall = termrender._termrender.GameTexture([pt_wall])
        
        # Spawn Walls (kept for visual aesthetic, snake can wrap through them)
        self.wall_ids = []
        for x in range(width):
            self.wall_ids.append(self.engine.spawn_object(x, 0, "wall", self.tex_wall))
            self.wall_ids.append(self.engine.spawn_object(x, height-1, "wall", self.tex_wall))
        for y in range(1, height-1):
            self.wall_ids.append(self.engine.spawn_object(0, y, "wall", self.tex_wall))
            self.wall_ids.append(self.engine.spawn_object(width-1, y, "wall", self.tex_wall))

        # Core Game State
        self.snake_pos = [(width//2, height//2)]
        self.snake_ids = [self.engine.spawn_object(width//2, height//2, "snake", self.tex_head)]
        
        self.direction = (1, 0)
        
        self.food_pos = self._spawn_food_pos()
        self.food_id = self.engine.spawn_object(self.food_pos[0], self.food_pos[1], "food", self.tex_food)

        self.score = 0
        self.game_over = False
        
        self.api.log_info("SnakeGame Initialized")

    def _spawn_food_pos(self) -> tuple[int, int]:
        while True:
            pos = (random.randint(1, self.width-2), random.randint(1, self.height-2))
            if pos not in self.snake_pos:
                return pos

    def handle_input(self):
        # We process keys if they are pressed
        if self.api.is_key_pressed("Up") and self.direction != (0, 1):
            self.direction = (0, -1)
        elif self.api.is_key_pressed("Down") and self.direction != (0, -1):
            self.direction = (0, 1)
        elif self.api.is_key_pressed("Left") and self.direction != (1, 0):
            self.direction = (-1, 0)
        elif self.api.is_key_pressed("Right") and self.direction != (-1, 0):
            self.direction = (1, 0)

    def update(self):
        if self.game_over: return
        
        head_x, head_y = self.snake_pos[0]
        dx, dy = self.direction
        new_head = (head_x + dx, head_y + dy)

        # 1. Wall collision (wrap around instead)
        # We need to wrap inside the visual walls which are at 0 and width-1/height-1.
        # So valid space is 1 to width-2.
        nx, ny = new_head
        if nx <= 0: nx = self.width - 2
        elif nx >= self.width - 1: nx = 1
        
        if ny <= 0: ny = self.height - 2
        elif ny >= self.height - 1: ny = 1
        
        new_head = (nx, ny)

        # 2. Self collision
        if new_head in self.snake_pos:
            self.game_over = True
            return

        self.snake_pos.insert(0, new_head)
        
        # 3. Eating food
        if new_head == self.food_pos:
            self.score += 10
            # update food
            self.food_pos = self._spawn_food_pos()
            self.api.set_location(self.food_id, self.food_pos[0], self.food_pos[1])
            self.api.force_rerender(self.food_id)
            
            # grow snake
            # To make the body correctly use the green body texture instead of the red head texture
            new_id = self.engine.spawn_object(new_head[0], new_head[1], "snake", self.tex_body)
            self.snake_ids.insert(0, new_id)
        else:
            self.snake_pos.pop()
            # Move the tail physical object to the front and update pos
            tail_id = self.snake_ids.pop()
            self.snake_ids.insert(0, tail_id)
            
            # update all coordinates in the backend
            for i, obj_id in enumerate(self.snake_ids):
                pos = self.snake_pos[i]
                self.api.set_location(obj_id, pos[0], pos[1])


def main():
    print(f"Starting Termrender Snake Engine (v{termrender.get_version()})...")
    
    with termrender.Engine("Snake Python Edition - PyO3", 60.0, 60.0, 60.0) as engine:
        engine.width = 40
        engine.height = 20
        api = engine.get_api()
        
        game = SnakeGame(engine, width=40, height=20)
        
        try:
            while not game.game_over:
                # 1. Input and rendering are baked into engine.poll_events() and game system!
                engine.poll_events()
                
                # Check exit
                if api.is_key_pressed("Esc"):
                    break
                    
                game.handle_input()
                game.update()
                
                # 3. Render
                engine.render()
                
                # Sleep controls our speed.
                time.sleep(0.1)
                
            api.log_info(f"Game Over! Final Score: {game.score}")
            engine.render() # One final render push
            time.sleep(1)
            
        except KeyboardInterrupt:
            api.log_info("Keyboard interrupt received.")
        except Exception as e:
            api.log_error(f"FATAL PYTHON EXCEPTION: {e}")
            print(f"EXCEPTION: {e}")
            raise

if __name__ == "__main__":
    main()
