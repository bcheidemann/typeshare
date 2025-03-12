package proto

import (
    "encoding/json"
    "time"
)

// This is a Person struct with camelCase rename
type Person struct {
	FirstName string `json:"firstName"`
	LastName string `json:"lastName"`
	Age int `json:"age"`
	ExtraSpecialField1 int `json:"extraSpecialField1"`
	ExtraSpecialField2 *[]string `json:"extraSpecialField2,omitempty"`
}
// This is a Person2 struct with UPPERCASE rename
type Person2 struct {
	FirstName string `json:"FIRST_NAME"`
	LastName string `json:"LAST_NAME"`
	Age int `json:"AGE"`
}
