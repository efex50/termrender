import sys
import termrender
import time

def main():
    print("Initializing PyO3 Termrender Demo...")
    engine = termrender._termrender.Engine("Demo PyO3 v0.1.0", 60.0, 60.0, 60.0)
    
    # We create some PyGColor and PyTermPrint
    color_red = termrender._termrender.GColor.red()
    color_blue = termrender._termrender.GColor.blue()
    color_green = termrender._termrender.GColor.green()
    
    text_red = termrender._termrender.TermPrint("O", color_red, None)
    text_blue = termrender._termrender.TermPrint("H", color_blue, None)
    text_green = termrender._termrender.TermPrint("X", color_green, None)
    
    # We construct a PyPrintType string
    pt_string = termrender._termrender.PrintType.new_string(0, 0, text_red)
    
    # Create the texture
    texture = termrender._termrender.GameTexture([pt_string])
    
    # Spawn an object using the helper
    obj_id = engine.spawn_object(10, 10, "player", texture)
    
    api = engine.get_api()
    api.log_info(f"Spawned player at ID {obj_id}")
    
    print("Entering game loop. Press ESC to exit.")
    
    # Let's run a simple mock loop 
    # Usually this would loop based on the engine's internal tick
    for i in range(100):
        # Poll events
        engine.poll_events()
        
        # update object location 
        api.set_location(obj_id, 10 + i % 20, 10 + i % 5)
        
        if api.is_key_pressed("Esc"):
            api.log_info("Escape pressed. Exiting.")
            break
            
        # Render
        engine.clear_screen()
        engine.render()
        
        time.sleep(0.05)
        
    print("Demo Finished.")

if __name__ == "__main__":
    main()
