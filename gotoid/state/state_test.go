package state_test

import (
	"."
	"testing"
)

func TestExampleSuccess(t *testing.T) {
	var initial_store = make(map[string]int)
	var store = state.NewStore(&initial_store)
}
