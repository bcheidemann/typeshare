@file:NoLiveLiterals

package com.agilebits.onepassword

import androidx.compose.runtime.NoLiveLiterals
import kotlinx.serialization.*

/// Generated type representing the anonymous struct variant `List` of the `AnonymousStructWithRename` Rust enum
@Serializable
data class AnonymousStructWithRenameListInner (
	val list: List<String>
)

/// Generated type representing the anonymous struct variant `LongFieldNames` of the `AnonymousStructWithRename` Rust enum
@Serializable
data class AnonymousStructWithRenameLongFieldNamesInner (
	val some_long_field_name: String,
	val and: Boolean,
	val but_one_more: List<String>
)

/// Generated type representing the anonymous struct variant `KebabCase` of the `AnonymousStructWithRename` Rust enum
@Serializable
data class AnonymousStructWithRenameKebabCaseInner (
	@SerialName("another-list")
	val another_list: List<String>,
	@SerialName("camelCaseStringField")
	val camelCaseStringField: String,
	@SerialName("something-else")
	val something_else: Boolean
)

@Serializable
sealed class AnonymousStructWithRename {
	@Serializable
	@SerialName("list")
	data class List(val content: AnonymousStructWithRenameListInner): AnonymousStructWithRename()
	@Serializable
	@SerialName("longFieldNames")
	data class LongFieldNames(val content: AnonymousStructWithRenameLongFieldNamesInner): AnonymousStructWithRename()
	@Serializable
	@SerialName("kebabCase")
	data class KebabCase(val content: AnonymousStructWithRenameKebabCaseInner): AnonymousStructWithRename()
}

