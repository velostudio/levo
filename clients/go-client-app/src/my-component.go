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
var heartOffset = [2]float32{0.0, 0.0}

func createParticles() {
    canvasSize := world.LevoPortalMyImportsCanvasSize()
    canvasWidth := canvasSize.Width
    tick++

    leftMouseButton := world.LevoPortalMyImportsMouseButtonLeft()

    if tick%10 == 0 {
        if len(particles) < 100 {
            newParticle := particle{
                x:      rand.Float32() * canvasWidth,
                y:      0,
                speed:  500 + rand.Float32()*13,
                radius: 5 + rand.Float32()*5,
                color:  "white",
            }

            cursorPositionOption := world.LevoPortalMyImportsCursorPosition()

            if cursorPositionOption.IsSome() && world.LevoPortalMyImportsMouseButtonPressed(leftMouseButton) {
                cursorPosition := cursorPositionOption.Unwrap()
                newParticle.x = cursorPosition.X
                newParticle.y = cursorPosition.Y
            }

            particles = append(particles, newParticle)
        }
    }
}

func updateParticles() {
        for i := 0; i < len(particles); i++ {
                particle := &particles[i]
                particle.y -= particle.speed * world.LevoPortalMyImportsDeltaSeconds()
        }
}

func killParticles() {
	var canvas_height float32 = world.LevoPortalMyImportsCanvasSize().Height
        for i := 0; i < len(particles); i++ {
                particle := &particles[i]
                if particle.y < -canvas_height {
                        particle.y = 0
                }
        }
}

func drawParticles() {
	var canvas_width float32 = world.LevoPortalMyImportsCanvasSize().Width
	var canvas_height float32 = world.LevoPortalMyImportsCanvasSize().Height
        world.LevoPortalMyImportsFillStyle("royal_purple")
        world.LevoPortalMyImportsFillRect(0, 0, canvas_width, canvas_height)
        for i := 0; i < len(particles); i++ {
                particle := &particles[i]
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

func drawHeart(xOffset float32, yOffset float32) {
    world.LevoPortalMyImportsBeginPath()
    world.LevoPortalMyImportsMoveTo(xOffset, yOffset)
    world.LevoPortalMyImportsCubicBezierTo(xOffset+70, yOffset+70, xOffset+175, yOffset-35, xOffset, yOffset-140)
    world.LevoPortalMyImportsCubicBezierTo(xOffset-175, yOffset-35, xOffset-70, yOffset+70, xOffset, yOffset)
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
        heartSpeed := float32(222.0)
        
	    if world.LevoPortalMyImportsKeyPressed(world.LevoPortalMyImportsKeyCodeLeft()) {
            heartOffset[0] -= heartSpeed * world.LevoPortalMyImportsDeltaSeconds()
        }
        
	    if world.LevoPortalMyImportsKeyPressed(world.LevoPortalMyImportsKeyCodeRight()) {
            heartOffset[0] += heartSpeed * world.LevoPortalMyImportsDeltaSeconds()
        }
        
	    if world.LevoPortalMyImportsKeyPressed(world.LevoPortalMyImportsKeyCodeUp()) {
            heartOffset[1] += heartSpeed * world.LevoPortalMyImportsDeltaSeconds()
        }
        
	    if world.LevoPortalMyImportsKeyPressed(world.LevoPortalMyImportsKeyCodeDown()) {
            heartOffset[1] -= heartSpeed * world.LevoPortalMyImportsDeltaSeconds()
        }
        
        drawHeart(heartOffset[0], heartOffset[1])
    }

    if tick > 200 {
        world.LevoPortalMyImportsLabel("Happy New Year from Go!", 0.0, -200.0, 64.0, "white")
        world.LevoPortalMyImportsLink("../rust.wasm", "Go to rust.wasm", -100.0, -300.0, 32.0)
    }
}

func main() {}
