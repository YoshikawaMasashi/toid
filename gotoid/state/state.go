package state

import (
	"fmt"
)

type Store struct {
	state interface{}
}

func NewStore(initial_state interface{}) Store {
	return Store{initial_state}
}

func (self *Store) update_state(state interface{}) {
	self.state = state
}

func (self *Store) GetState() interface{} {
	return self.state
}

type Reducer struct {
	store       *Store
	reduce_func func(interface{}, interface{}) interface{}
}

func (self *Reducer) reduce(event interface{}) {
	var state = self.store.GetState()
	var new_state = self.reduce_func(state, event)
	self.store.update_state(new_state)
}

func PrintHelloWorld() {
	fmt.Println("hello world")
}
