// my-component.go
package main

import (
	"math"
	"math/rand"
	world "workspace/my-world"
)

type particle struct {
	x      float32
	y      float32
	speed  float32
	radius float32
	color  string
}

var particles []particle

var tick int = 0

func createParticles() {
	var canvas_width float32 = 1200 // TODO: pass from host
	tick += 1
	if tick%10 == 0 {
		if len(particles) < 100 {
			particles = append(particles, particle{
				x:      rand.Float32() * canvas_width,
				y:      0,
				speed:  500 + rand.Float32()*13,
				radius: 5 + rand.Float32()*5,
				color:  "white",
			})
		}
	}
}

func updateParticles() {
	for i := 0; i < len(particles); i++ {
		particle := particles[i]
		particle.y -= particle.speed * world.LevoPortalMyImportsDeltaSeconds()
	}
}

func killParticles() {
	var canvas_height float32 = 800 // TODO: pass from host
	for i := 0; i < len(particles); i++ {
		particle := particles[i]
		if particle.y < -canvas_height {
			particle.y = 0
		}
	}
}

func drawParticles() {
	// TODO: provide canvas interface on wit level, something like
	//   interface canvas {
	//     type canvas-id = u64;
	//     record point {
	//         x: u32,
	//         y: u32,
	//     }
	//     draw-line: func(canvas: canvas-id, from: point, to: point);
	// }
	world.LevoPortalMyImportsFillStyle("royal_purple")
	world.LevoPortalMyImportsFillRect(0, 0, 1200, 800)
	for i := 0; i < len(particles); i++ {
		particle := particles[i]
		world.LevoPortalMyImportsBeginPath()
		world.LevoPortalMyImportsArc(
			particle.x,
			particle.y,
			particle.radius,
			2*math.Pi,
			0,
		)
		world.LevoPortalMyImportsClosePath()
		world.LevoPortalMyImportsFillStyle(particle.color)
		world.LevoPortalMyImportsFill()
	}
}

func drawHeart() {
	world.LevoPortalMyImportsBeginPath()
	world.LevoPortalMyImportsMoveTo(0, 0)
	world.LevoPortalMyImportsCubicBezierTo(70, 70, 175, -35, 0, -140)
	world.LevoPortalMyImportsCubicBezierTo(-175, -35, -70, 70, 0, 0)
	world.LevoPortalMyImportsClosePath()
	world.LevoPortalMyImportsFillStyle("red")
	world.LevoPortalMyImportsFill()
}

func init() {
	a := HostImpl{}
	world.SetMyWorld(a)
}

type HostImpl struct {
}

func (e HostImpl) Setup() {
	world.LevoPortalMyImportsPrint("setup from guest (Go) has been called")
}

func (e HostImpl) Update() {
	createParticles()
	updateParticles()
	killParticles()
	drawParticles()
	if tick > 100 {
		drawHeart()
	}
	if tick > 200 {
		world.LevoPortalMyImportsLabel("Happy New Year from Go!", 0., -200., 64., "white")
	}
}

func main() {}
