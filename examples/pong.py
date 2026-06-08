#!/usr/bin/env python3
"""
Termrender Pong Game Demo (Duvarlı Versiyon)
"""

import time
import sys

try:
    import termrender
except ImportError:
    print("Hata: termrender modülü bulunamadı.")
    sys.exit(1)

class PongGame:
    def __init__(self, engine, width: int = 60, height: int = 20):
        self.engine = engine
        self.api = engine.get_api()
        self.width = width
        self.height = height
        
        # Renkler
        color_white = termrender._termrender.GColor.white()
        color_red = termrender._termrender.GColor.red()
        color_wall = termrender._termrender.GColor.white() # Duvar Rengi
        
        # Karakterler
        txt_paddle = termrender._termrender.TermPrint("█", color_white, None)
        txt_ball = termrender._termrender.TermPrint("O", color_red, None)
        txt_wall = termrender._termrender.TermPrint("#", color_wall, None) # Duvar Karakteri

        # Raket (Paddle) Dokusu
        pt_pad_top = termrender._termrender.PrintType.new_string(0, -1, txt_paddle)
        pt_pad_mid = termrender._termrender.PrintType.new_string(0, 0, txt_paddle)
        pt_pad_bot = termrender._termrender.PrintType.new_string(0, 1, txt_paddle)
        self.tex_paddle = termrender._termrender.GameTexture([pt_pad_top, pt_pad_mid, pt_pad_bot])
        
        # Top Dokusu
        pt_ball = termrender._termrender.PrintType.new_string(0, 0, txt_ball)
        self.tex_ball = termrender._termrender.GameTexture([pt_ball])

        # Duvar Dokusu
        pt_wall = termrender._termrender.PrintType.new_string(0, 0, txt_wall)
        self.tex_wall = termrender._termrender.GameTexture([pt_wall])

        # --- DUVARLARI OLUŞTURMA ---
        self.wall_ids = []
        # Alt ve Üst Duvarlar (Sekme alanları)
        for x in range(width):
            self.wall_ids.append(self.engine.spawn_object(x, 0, "wall_top", self.tex_wall))
            self.wall_ids.append(self.engine.spawn_object(x, height-1, "wall_bot", self.tex_wall))
            
        # Sol ve Sağ Duvarlar (Kale çizgileri)
        for y in range(1, height-1):
            self.wall_ids.append(self.engine.spawn_object(0, y, "wall_left", self.tex_wall))
            self.wall_ids.append(self.engine.spawn_object(width-1, y, "wall_right", self.tex_wall))

        # Başlangıç Durumları (Raketler duvarların içine taşmasın diye y limitleri 2 ile height-3 arası olacak)
        self.p1_y = height // 2
        self.p2_y = height // 2
        
        self.ball_x = width // 2
        self.ball_y = height // 2
        self.ball_dx = 1
        self.ball_dy = 1

        # Nesneleri Oyun Dünyasına Ekleme
        self.p1_id = self.engine.spawn_object(3, self.p1_y, "paddle1", self.tex_paddle)
        self.p2_id = self.engine.spawn_object(width - 4, self.p2_y, "paddle2", self.tex_paddle)
        self.ball_id = self.engine.spawn_object(self.ball_x, self.ball_y, "ball", self.tex_ball)

        self.score_p1 = 0
        self.score_p2 = 0
        
        self.api.log_info("PongGame Başlatıldı!")

    def handle_input(self):
        # Sağ Oyuncu (Player 2) -> Yön tuşları
        # Raketlerin üst/alt duvarlara taşmasını engellemek için sınırları güncelledik (2 ve height-3)
        if self.api.is_key_pressed("Up"):
            self.p2_y = max(2, self.p2_y - 1)
        elif self.api.is_key_pressed("Down"):
            self.p2_y = min(self.height - 3, self.p2_y + 1)

        # Sol Oyuncu (Player 1) -> Q (Yukarı) ve E (Aşağı)
        if self.api.is_key_pressed("Q"):
            self.p1_y = max(2, self.p1_y - 1)
        elif self.api.is_key_pressed("E"):
            self.p1_y = min(self.height - 3, self.p1_y + 1)

    def reset_ball(self):
        self.ball_x = self.width // 2
        self.ball_y = self.height // 2
        self.ball_dx *= -1 

    def update(self):
        # 1. Raketlerin konumunu güncelle
        self.api.set_location(self.p1_id, 3, self.p1_y)
        self.api.set_location(self.p2_id, self.width - 4, self.p2_y)

        # 2. Topun konumunu güncelle
        self.ball_x += self.ball_dx
        self.ball_y += self.ball_dy

        # 3. Duvar (Alt/Üst) Çarpışması (Artık y=1 ve y=height-2 sınırlarında sekecek)
        if self.ball_y <= 1 or self.ball_y >= self.height - 2:
            self.ball_dy *= -1
            
        # 4. Raket Çarpışmaları 
        # Sol Raket (x koordinatı 4, raket x koordinatı 3 olduğu için)
        if self.ball_x == 4 and abs(self.ball_y - self.p1_y) <= 1:
            self.ball_dx *= -1
            self.ball_x = 5 # Duvara yapışmayı engelle
            
        # Sağ Raket
        elif self.ball_x == self.width - 5 and abs(self.ball_y - self.p2_y) <= 1:
            self.ball_dx *= -1
            self.ball_x = self.width - 6

        # 5. Skor (Top sol/sağ duvara çarptığında)
        if self.ball_x <= 0:
            self.score_p2 += 1
            self.api.log_info(f"P2 Skor Yaptı! Durum: P1 {self.score_p1} - {self.score_p2} P2")
            self.reset_ball()
        elif self.ball_x >= self.width - 1:
            self.score_p1 += 1
            self.api.log_info(f"P1 Skor Yaptı! Durum: P1 {self.score_p1} - {self.score_p2} P2")
            self.reset_ball()

        # Top konumunu motora bildir
        self.api.set_location(self.ball_id, self.ball_x, self.ball_y)


def main():
    print("Termrender Pong Başlıyor...")
    
    with termrender.Engine("Pong Python Edition - PyO3", 60.0, 60.0, 60.0) as engine:
        engine.width = 60
        engine.height = 20
        api = engine.get_api()
        
        game = PongGame(engine, width=engine.width, height=engine.height)
        
        try:
            while True:
                engine.poll_events()
                
                if api.is_key_pressed("Esc"):
                    break
                    
                game.handle_input()
                game.update()
                
                engine.render()
                time.sleep(0.05)
                
            api.log_info(f"Oyun Bitti! Nihai Skor -> P1: {game.score_p1} | P2: {game.score_p2}")
            
        except KeyboardInterrupt:
            api.log_info("Kullanıcı tarafından durduruldu.")

if __name__ == "__main__":
    main()