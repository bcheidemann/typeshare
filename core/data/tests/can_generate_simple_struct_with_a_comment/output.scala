package com.agilebits

package object onepassword {

type UByte = Byte
type UShort = Short
type UInt = Int
type ULong = Int

}
package onepassword {

class Location extends Serializable

// This is a comment.
case class Person (
	// This is another comment
	name: String,
	age: UByte,
	info: Option[String] = None,
	emails: Vector[String],
	location: Location
)

}
