/*
    supaterm – terminal manipulation library allowing use of colored text and other functionality is planned
    Copyright (C) 2026  @polyagonal1

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>
*/

macro_rules! __fill_type {
    (, $default_ty:ty) => {
        $default_ty
    };
    ($some_ty:ty, $default_ty:ty) => {
        $some_ty
    };
}

macro_rules! define {
    (default-no-args
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident,
        capability: $capability:expr,
        capability_getter: $capability_getter:expr,
        size_hint: $size:expr,
        unsupported_msg: $unsupported_msg:literal $(,)?
        $(--add-command-implementation-errors-docs $($placeholder:tt)? )?
    ) => {
        define!(custom-impl
            $(#[$attrs])*
            definition: $visible struct $typ,
            capability: $capability,
            capability_getter: $capability_getter,
            size_hint: $size,
            unsupported_msg: $unsupported_msg,
            write_to_impl: |self, database, capability_data, target| {

                let res = infoterm::expand::expand(capability_data, &[]);

                let expanded = match res {
                    Ok(expanded) => expanded,
                    Err(_) => return Err(::std::io::ErrorKind::InvalidData.into()),
                };

                target.write_all(expanded.as_slice())?;
            },
            is_supported_impl: |self, database, capability| {
                // this is ok because whether the capability exists is checked within the call
                true
            }
            $(--add-command-implementation-errors-docs $($placeholder)?)?
        );
    };
    (custom-impl
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident $(( $( $args:tt  )+ ))?,
        capability: $capability:expr,
        capability_getter: $capability_getter:expr,
        size_hint: $size_hint:expr,
        unsupported_msg: $unsupported_msg:literal,
        write_to_impl: |$write_to_self_var_name:ident, $write_to_entry_var_name:ident, $cap_var_name:ident, $target_var_name:ident $(,)?| $write_to_impl:block,
        is_supported_impl: |$is_supported_self_var_name:ident $(: $is_supported_self_var_ty:ty)?, $is_supported_entry_var_name:ident $(: $is_supported_entry_var_ty:ty)?, $is_supported_capability_var_name:ident $(: $is_supported_capability_var_ty:ty)? $(,)?| $is_supported_impl:block $(,)?
    ) => {

        $(#[$attrs])*
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        $visible struct $typ $(( $($args)+ ))?;

        impl $crate::Command for $typ {
            fn size_hint(&self) -> Option<usize> {
                $size_hint
            }

            fn write_to(
                $write_to_self_var_name: &Self,
                $write_to_entry_var_name: &::infoterm::entry::Entry,
                #[allow(unused)] $target_var_name: &mut dyn ::std::io::Write
            ) -> ::std::io::Result<()> {
                match $capability_getter($write_to_entry_var_name, $capability.into()) {
                    // this command is supported
                    Some($cap_var_name) => $write_to_impl,
                    // this command is unsupported
                    None => return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::Unsupported,
                        $unsupported_msg
                    ))
                }
                Ok(())
            }
        }

        impl $crate::Capability for $typ {

            fn is_supported(
                $is_supported_self_var_name: $crate::__fill_type!($($is_supported_self_var_ty)?, &Self),
                $is_supported_entry_var_name: $crate::__fill_type!($($is_supported_entry_var_ty)?, &::infoterm::entry::Entry),
            ) -> bool {
                match $capability_getter($is_supported_entry_var_name, $capability.into()) {
                    #[allow(unused)] Some($is_supported_capability_var_name) => $is_supported_impl,
                    None => false
                }
            }
        }
    };
}

macro_rules! add_semicolon_if_unit_or_tuple_struct {
    // normal struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)? {$(
            $first_field:ident: $first_field_ty:ty
            $(
                ,$field:ident: $field_ty:ty
            )* $(,)?
        )?}
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)? {$(
            $first_field: $first_field_ty
            $(
                ,$field: $field_ty
            )* $(,)?
        )?}
    };
    // tuple struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)? $(($( $first_field:ty $(,$fields:ty)* $(,)? )?))?
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)? $(($( $first_field $(,$fields)* $(,)? )?))?;
    };
    // unit struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)?
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)?;
    }
}

macro_rules! new_define {
    (
        $(#[$attrs:meta])*
        $visibility:vis command $ty:ident $(<$($generic_params:tt)*>)? $($fields:tt)* {
            $(#[$write_to_attrs:meta])*
            fn write_to($self_var_name:ident: &Self) $write_to_impl:block

            $(#[$is_supported_attrs:meta])*
            fn is_supported $is_supported_args:tt $is_supported_impl:block

            $(#[$size_hint_attrs:meta])*
            fn size_hint $size_hint_args:tt $size_hint_impl:block
        }
    ) => {
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        add_semicolon_if_unit_or_tuple_struct! {
            $visibility struct $ty $(<$($generic_params)*>)? $($fields)*
        }

        impl $crate::Command for $ty {

            fn size_hint(&self) -> Option<usize> $size_hint_impl

            fn is_supported(&self, database: &$crate::terminfo::Database) -> bool $is_supported_impl

            fn write_to(&self, database: &Database, ctx: &mut Context, target: &mut dyn ::std::io::Write) -> ::std::io::error::Result<()> {

            }
        }
    }
}

pub(crate) use {__fill_type, define, new_define};
