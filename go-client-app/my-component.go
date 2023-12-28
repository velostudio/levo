// my-component.go
package main

import (
    gen "workspace/my-world"
)

func init() {
    a := HostImpl{}
    gen.SetMyWorld(a)
}

type HostImpl struct {
}

func (e HostImpl) Setup() {
}

func (e HostImpl) Update() {
  gen.LevoPortalMyImportsLabel("Hello, from Go!", 100.0, 100.0, 60.0, "white")
}

func main() {}
