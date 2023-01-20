pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-0.3.1");

/// The root namespace
///
/// Generated from these locations:
/// * File `auth/auth.fbs`
#[no_implicit_prelude]
mod root {
    /// The namespace `Auth`
    ///
    /// Generated from these locations:
    /// * File `auth/auth.fbs`
    pub mod auth {
        /// The namespace `Auth.WebAuthn`
        ///
        /// Generated from these locations:
        /// * File `auth/webauthn.fbs`
        pub mod web_authn {
            /// The enum `AttestationConveyancePreference` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Enum `AttestationConveyancePreference` in the file `auth/webauthn.fbs:3`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i8)]
            pub enum AttestationConveyancePreference {
                ///  Do not request attestation.
                ///  <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-none>
                None = 0,

                ///  Request attestation in a semi-anonymized form.
                ///  <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-indirect>
                Indirect = 1,

                ///  Request attestation in a direct form.
                ///  <https://www.w3.org/TR/webauthn/#dom-attestationconveyancepreference-direct>
                Direct = 2,
            }

            impl AttestationConveyancePreference {
                /// Array containing all valid variants of AttestationConveyancePreference
                pub const ENUM_VALUES: [Self; 3] = [Self::None, Self::Indirect, Self::Direct];
            }

            impl ::core::convert::TryFrom<i8> for AttestationConveyancePreference {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(AttestationConveyancePreference::None),
                        1 => ::core::result::Result::Ok(AttestationConveyancePreference::Indirect),
                        2 => ::core::result::Result::Ok(AttestationConveyancePreference::Direct),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<AttestationConveyancePreference> for i8 {
                #[inline]
                fn from(value: AttestationConveyancePreference) -> Self {
                    value as i8
                }
            }

            impl ::planus::Primitive for AttestationConveyancePreference {
                const ALIGNMENT: usize = 1;
                const SIZE: usize = 1;
            }

            impl ::planus::WriteAsPrimitive<AttestationConveyancePreference>
                for AttestationConveyancePreference
            {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i8).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<AttestationConveyancePreference> for AttestationConveyancePreference {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> AttestationConveyancePreference {
                    *self
                }
            }

            impl
                ::planus::WriteAsDefault<
                    AttestationConveyancePreference,
                    AttestationConveyancePreference,
                > for AttestationConveyancePreference
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &AttestationConveyancePreference,
                ) -> ::core::option::Option<AttestationConveyancePreference> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<AttestationConveyancePreference>
                for AttestationConveyancePreference
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<AttestationConveyancePreference> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for AttestationConveyancePreference {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for AttestationConveyancePreference {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 1;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value = *buffer.buffer.get_unchecked(offset) as i8;
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "AttestationConveyancePreference",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<AttestationConveyancePreference> for AttestationConveyancePreference {
                const STRIDE: usize = 1;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - i as u32,
                        );
                    }
                }
            }

            /// The table `CredentialCreationOptions` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `CredentialCreationOptions` in the file `auth/webauthn.fbs:17`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct CredentialCreationOptions {
                ///  The relying party
                pub rp: ::core::option::Option<::planus::alloc::boxed::Box<self::RelyingParty>>,
                ///  The user.
                pub user: ::core::option::Option<::planus::alloc::boxed::Box<self::User>>,
                ///  The one-time challenge for the credential to sign
                pub challenge: ::core::option::Option<::planus::alloc::string::String>,
                ///  The set of cryptographic types allowed by this server.
                pub pub_key_cred_params:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::PubKeyCredParams>>,
                ///  The timeout for the authenticator to stop accepting the operation
                pub timeout: u32,
                ///  The requested attestation level from the device.
                pub attestation: self::AttestationConveyancePreference,
                ///  The list of credentials that are excluded from this operation.
                pub exclude_credentials: ::core::option::Option<
                    ::planus::alloc::vec::Vec<self::PublicKeyCredentialDescriptor>,
                >,
                ///  The list of authenticators that are allowed for this operation.
                pub authenticator_selection: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::AuthenticatorSelectionCriteria>,
                >,
                ///  The extensions that are allowed for this operation.
                pub extensions: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::RequestRegistrationExtensions>,
                >,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for CredentialCreationOptions {
                fn default() -> Self {
                    Self {
                        rp: ::core::default::Default::default(),
                        user: ::core::default::Default::default(),
                        challenge: ::core::default::Default::default(),
                        pub_key_cred_params: ::core::default::Default::default(),
                        timeout: 60000,
                        attestation: self::AttestationConveyancePreference::None,
                        exclude_credentials: ::core::default::Default::default(),
                        authenticator_selection: ::core::default::Default::default(),
                        extensions: ::core::default::Default::default(),
                    }
                }
            }

            impl CredentialCreationOptions {
                /// Creates a [CredentialCreationOptionsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> CredentialCreationOptionsBuilder<()> {
                    CredentialCreationOptionsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_rp: impl ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                    field_user: impl ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                    field_challenge: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                    field_pub_key_cred_params: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>,
                    >,
                    field_timeout: impl ::planus::WriteAsDefault<u32, u32>,
                    field_attestation: impl ::planus::WriteAsDefault<
                        self::AttestationConveyancePreference,
                        self::AttestationConveyancePreference,
                    >,
                    field_exclude_credentials: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PublicKeyCredentialDescriptor>]>,
                    >,
                    field_authenticator_selection: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorSelectionCriteria>,
                    >,
                    field_extensions: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::RequestRegistrationExtensions>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_rp = field_rp.prepare(builder);
                    let prepared_user = field_user.prepare(builder);
                    let prepared_challenge = field_challenge.prepare(builder);
                    let prepared_pub_key_cred_params = field_pub_key_cred_params.prepare(builder);
                    let prepared_timeout = field_timeout.prepare(builder, &60000);
                    let prepared_attestation = field_attestation
                        .prepare(builder, &self::AttestationConveyancePreference::None);
                    let prepared_exclude_credentials = field_exclude_credentials.prepare(builder);
                    let prepared_authenticator_selection =
                        field_authenticator_selection.prepare(builder);
                    let prepared_extensions = field_extensions.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<22> =
                        ::core::default::Default::default();
                    if prepared_rp.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::RelyingParty>>(0);
                    }
                    if prepared_user.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::User>>(1);
                    }
                    if prepared_challenge.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(2);
                    }
                    if prepared_pub_key_cred_params.is_some() {
                        table_writer.write_entry::<::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>>(3);
                    }
                    if prepared_timeout.is_some() {
                        table_writer.write_entry::<u32>(4);
                    }
                    if prepared_exclude_credentials.is_some() {
                        table_writer.write_entry::<::planus::Offset<
                            [::planus::Offset<self::PublicKeyCredentialDescriptor>],
                        >>(6);
                    }
                    if prepared_authenticator_selection.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<self::AuthenticatorSelectionCriteria>>(
                                7,
                            );
                    }
                    if prepared_extensions.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<self::RequestRegistrationExtensions>>(
                                8,
                            );
                    }
                    if prepared_attestation.is_some() {
                        table_writer.write_entry::<self::AttestationConveyancePreference>(5);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_rp) = prepared_rp {
                                object_writer.write::<_, _, 4>(&prepared_rp);
                            }
                            if let ::core::option::Option::Some(prepared_user) = prepared_user {
                                object_writer.write::<_, _, 4>(&prepared_user);
                            }
                            if let ::core::option::Option::Some(prepared_challenge) =
                                prepared_challenge
                            {
                                object_writer.write::<_, _, 4>(&prepared_challenge);
                            }
                            if let ::core::option::Option::Some(prepared_pub_key_cred_params) =
                                prepared_pub_key_cred_params
                            {
                                object_writer.write::<_, _, 4>(&prepared_pub_key_cred_params);
                            }
                            if let ::core::option::Option::Some(prepared_timeout) = prepared_timeout
                            {
                                object_writer.write::<_, _, 4>(&prepared_timeout);
                            }
                            if let ::core::option::Option::Some(prepared_exclude_credentials) =
                                prepared_exclude_credentials
                            {
                                object_writer.write::<_, _, 4>(&prepared_exclude_credentials);
                            }
                            if let ::core::option::Option::Some(prepared_authenticator_selection) =
                                prepared_authenticator_selection
                            {
                                object_writer.write::<_, _, 4>(&prepared_authenticator_selection);
                            }
                            if let ::core::option::Option::Some(prepared_extensions) =
                                prepared_extensions
                            {
                                object_writer.write::<_, _, 4>(&prepared_extensions);
                            }
                            if let ::core::option::Option::Some(prepared_attestation) =
                                prepared_attestation
                            {
                                object_writer.write::<_, _, 1>(&prepared_attestation);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<CredentialCreationOptions>> for CredentialCreationOptions {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredentialCreationOptions> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<CredentialCreationOptions>>
                for CredentialCreationOptions
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredentialCreationOptions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<CredentialCreationOptions> for CredentialCreationOptions {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredentialCreationOptions> {
                    CredentialCreationOptions::create(
                        builder,
                        &self.rp,
                        &self.user,
                        &self.challenge,
                        &self.pub_key_cred_params,
                        self.timeout,
                        self.attestation,
                        &self.exclude_credentials,
                        &self.authenticator_selection,
                        &self.extensions,
                    )
                }
            }

            /// Builder for serializing an instance of the [CredentialCreationOptions] type.
            ///
            /// Can be created using the [CredentialCreationOptions::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct CredentialCreationOptionsBuilder<State>(State);

            impl CredentialCreationOptionsBuilder<()> {
                /// Setter for the [`rp` field](CredentialCreationOptions#structfield.rp).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn rp<T0>(self, value: T0) -> CredentialCreationOptionsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                {
                    CredentialCreationOptionsBuilder((value,))
                }

                /// Sets the [`rp` field](CredentialCreationOptions#structfield.rp) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn rp_as_null(self) -> CredentialCreationOptionsBuilder<((),)> {
                    self.rp(())
                }
            }

            impl<T0> CredentialCreationOptionsBuilder<(T0,)> {
                /// Setter for the [`user` field](CredentialCreationOptions#structfield.user).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn user<T1>(self, value: T1) -> CredentialCreationOptionsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                {
                    let (v0,) = self.0;
                    CredentialCreationOptionsBuilder((v0, value))
                }

                /// Sets the [`user` field](CredentialCreationOptions#structfield.user) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn user_as_null(self) -> CredentialCreationOptionsBuilder<(T0, ())> {
                    self.user(())
                }
            }

            impl<T0, T1> CredentialCreationOptionsBuilder<(T0, T1)> {
                /// Setter for the [`challenge` field](CredentialCreationOptions#structfield.challenge).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn challenge<T2>(
                    self,
                    value: T2,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, value))
                }

                /// Sets the [`challenge` field](CredentialCreationOptions#structfield.challenge) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn challenge_as_null(self) -> CredentialCreationOptionsBuilder<(T0, T1, ())> {
                    self.challenge(())
                }
            }

            impl<T0, T1, T2> CredentialCreationOptionsBuilder<(T0, T1, T2)> {
                /// Setter for the [`pub_key_cred_params` field](CredentialCreationOptions#structfield.pub_key_cred_params).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn pub_key_cred_params<T3>(
                    self,
                    value: T3,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>,
                    >,
                {
                    let (v0, v1, v2) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, value))
                }

                /// Sets the [`pub_key_cred_params` field](CredentialCreationOptions#structfield.pub_key_cred_params) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn pub_key_cred_params_as_null(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, ())> {
                    self.pub_key_cred_params(())
                }
            }

            impl<T0, T1, T2, T3> CredentialCreationOptionsBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`timeout` field](CredentialCreationOptions#structfield.timeout).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn timeout<T4>(
                    self,
                    value: T4,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsDefault<u32, u32>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`timeout` field](CredentialCreationOptions#structfield.timeout) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn timeout_as_default(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)>
                {
                    self.timeout(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4)> {
                /// Setter for the [`attestation` field](CredentialCreationOptions#structfield.attestation).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn attestation<T5>(
                    self,
                    value: T5,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5)>
                where
                    T5: ::planus::WriteAsDefault<
                        self::AttestationConveyancePreference,
                        self::AttestationConveyancePreference,
                    >,
                {
                    let (v0, v1, v2, v3, v4) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, v3, v4, value))
                }

                /// Sets the [`attestation` field](CredentialCreationOptions#structfield.attestation) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn attestation_as_default(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, ::planus::DefaultValue)>
                {
                    self.attestation(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4, T5> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5)> {
                /// Setter for the [`exclude_credentials` field](CredentialCreationOptions#structfield.exclude_credentials).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn exclude_credentials<T6>(
                    self,
                    value: T6,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6)>
                where
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PublicKeyCredentialDescriptor>]>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, v3, v4, v5, value))
                }

                /// Sets the [`exclude_credentials` field](CredentialCreationOptions#structfield.exclude_credentials) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn exclude_credentials_as_null(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, ())>
                {
                    self.exclude_credentials(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6)> {
                /// Setter for the [`authenticator_selection` field](CredentialCreationOptions#structfield.authenticator_selection).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn authenticator_selection<T7>(
                    self,
                    value: T7,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
                where
                    T7: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorSelectionCriteria>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5, v6) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, v3, v4, v5, v6, value))
                }

                /// Sets the [`authenticator_selection` field](CredentialCreationOptions#structfield.authenticator_selection) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn authenticator_selection_as_null(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, ())>
                {
                    self.authenticator_selection(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7>
                CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7)>
            {
                /// Setter for the [`extensions` field](CredentialCreationOptions#structfield.extensions).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn extensions<T8>(
                    self,
                    value: T8,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
                where
                    T8: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RequestRegistrationExtensions>,
                    >,
                {
                    let (v0, v1, v2, v3, v4, v5, v6, v7) = self.0;
                    CredentialCreationOptionsBuilder((v0, v1, v2, v3, v4, v5, v6, v7, value))
                }

                /// Sets the [`extensions` field](CredentialCreationOptions#structfield.extensions) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn extensions_as_null(
                    self,
                ) -> CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, ())>
                {
                    self.extensions(())
                }
            }

            impl<T0, T1, T2, T3, T4, T5, T6, T7, T8>
                CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
            {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [CredentialCreationOptions].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredentialCreationOptions>
                where
                    Self: ::planus::WriteAsOffset<CredentialCreationOptions>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                    T5: ::planus::WriteAsDefault<
                        self::AttestationConveyancePreference,
                        self::AttestationConveyancePreference,
                    >,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PublicKeyCredentialDescriptor>]>,
                    >,
                    T7: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorSelectionCriteria>,
                    >,
                    T8: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RequestRegistrationExtensions>,
                    >,
                > ::planus::WriteAs<::planus::Offset<CredentialCreationOptions>>
                for CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
            {
                type Prepared = ::planus::Offset<CredentialCreationOptions>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredentialCreationOptions> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                    T5: ::planus::WriteAsDefault<
                        self::AttestationConveyancePreference,
                        self::AttestationConveyancePreference,
                    >,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PublicKeyCredentialDescriptor>]>,
                    >,
                    T7: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorSelectionCriteria>,
                    >,
                    T8: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RequestRegistrationExtensions>,
                    >,
                > ::planus::WriteAsOptional<::planus::Offset<CredentialCreationOptions>>
                for CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
            {
                type Prepared = ::planus::Offset<CredentialCreationOptions>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredentialCreationOptions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                    T5: ::planus::WriteAsDefault<
                        self::AttestationConveyancePreference,
                        self::AttestationConveyancePreference,
                    >,
                    T6: ::planus::WriteAsOptional<
                        ::planus::Offset<[::planus::Offset<self::PublicKeyCredentialDescriptor>]>,
                    >,
                    T7: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorSelectionCriteria>,
                    >,
                    T8: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RequestRegistrationExtensions>,
                    >,
                > ::planus::WriteAsOffset<CredentialCreationOptions>
                for CredentialCreationOptionsBuilder<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredentialCreationOptions> {
                    let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = &self.0;
                    CredentialCreationOptions::create(builder, v0, v1, v2, v3, v4, v5, v6, v7, v8)
                }
            }

            /// Reference to a deserialized [CredentialCreationOptions].
            #[derive(Copy, Clone)]
            pub struct CredentialCreationOptionsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> CredentialCreationOptionsRef<'a> {
                /// Getter for the [`rp` field](CredentialCreationOptions#structfield.rp).
                #[inline]
                pub fn rp(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::RelyingPartyRef<'a>>>
                {
                    self.0.access(0, "CredentialCreationOptions", "rp")
                }

                /// Getter for the [`user` field](CredentialCreationOptions#structfield.user).
                #[inline]
                pub fn user(&self) -> ::planus::Result<::core::option::Option<self::UserRef<'a>>> {
                    self.0.access(1, "CredentialCreationOptions", "user")
                }

                /// Getter for the [`challenge` field](CredentialCreationOptions#structfield.challenge).
                #[inline]
                pub fn challenge(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(2, "CredentialCreationOptions", "challenge")
                }

                /// Getter for the [`pub_key_cred_params` field](CredentialCreationOptions#structfield.pub_key_cred_params).
                #[inline]
                pub fn pub_key_cred_params(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<'a, ::planus::Result<self::PubKeyCredParamsRef<'a>>>,
                    >,
                > {
                    self.0
                        .access(3, "CredentialCreationOptions", "pub_key_cred_params")
                }

                /// Getter for the [`timeout` field](CredentialCreationOptions#structfield.timeout).
                #[inline]
                pub fn timeout(&self) -> ::planus::Result<u32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(4, "CredentialCreationOptions", "timeout")?
                            .unwrap_or(60000),
                    )
                }

                /// Getter for the [`attestation` field](CredentialCreationOptions#structfield.attestation).
                #[inline]
                pub fn attestation(
                    &self,
                ) -> ::planus::Result<self::AttestationConveyancePreference> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(5, "CredentialCreationOptions", "attestation")?
                            .unwrap_or(self::AttestationConveyancePreference::None),
                    )
                }

                /// Getter for the [`exclude_credentials` field](CredentialCreationOptions#structfield.exclude_credentials).
                #[inline]
                pub fn exclude_credentials(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<
                            'a,
                            ::planus::Result<self::PublicKeyCredentialDescriptorRef<'a>>,
                        >,
                    >,
                > {
                    self.0
                        .access(6, "CredentialCreationOptions", "exclude_credentials")
                }

                /// Getter for the [`authenticator_selection` field](CredentialCreationOptions#structfield.authenticator_selection).
                #[inline]
                pub fn authenticator_selection(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::AuthenticatorSelectionCriteriaRef<'a>>,
                > {
                    self.0
                        .access(7, "CredentialCreationOptions", "authenticator_selection")
                }

                /// Getter for the [`extensions` field](CredentialCreationOptions#structfield.extensions).
                #[inline]
                pub fn extensions(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::RequestRegistrationExtensionsRef<'a>>,
                > {
                    self.0.access(8, "CredentialCreationOptions", "extensions")
                }
            }

            impl<'a> ::core::fmt::Debug for CredentialCreationOptionsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("CredentialCreationOptionsRef");
                    if let ::core::option::Option::Some(field_rp) = self.rp().transpose() {
                        f.field("rp", &field_rp);
                    }
                    if let ::core::option::Option::Some(field_user) = self.user().transpose() {
                        f.field("user", &field_user);
                    }
                    if let ::core::option::Option::Some(field_challenge) =
                        self.challenge().transpose()
                    {
                        f.field("challenge", &field_challenge);
                    }
                    if let ::core::option::Option::Some(field_pub_key_cred_params) =
                        self.pub_key_cred_params().transpose()
                    {
                        f.field("pub_key_cred_params", &field_pub_key_cred_params);
                    }
                    f.field("timeout", &self.timeout());
                    f.field("attestation", &self.attestation());
                    if let ::core::option::Option::Some(field_exclude_credentials) =
                        self.exclude_credentials().transpose()
                    {
                        f.field("exclude_credentials", &field_exclude_credentials);
                    }
                    if let ::core::option::Option::Some(field_authenticator_selection) =
                        self.authenticator_selection().transpose()
                    {
                        f.field("authenticator_selection", &field_authenticator_selection);
                    }
                    if let ::core::option::Option::Some(field_extensions) =
                        self.extensions().transpose()
                    {
                        f.field("extensions", &field_extensions);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<CredentialCreationOptionsRef<'a>> for CredentialCreationOptions {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: CredentialCreationOptionsRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        rp: if let ::core::option::Option::Some(rp) = value.rp()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(rp)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        user: if let ::core::option::Option::Some(user) = value.user()? {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(user)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        challenge: if let ::core::option::Option::Some(challenge) =
                            value.challenge()?
                        {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(
                                challenge,
                            )?)
                        } else {
                            ::core::option::Option::None
                        },
                        pub_key_cred_params: if let ::core::option::Option::Some(
                            pub_key_cred_params,
                        ) = value.pub_key_cred_params()?
                        {
                            ::core::option::Option::Some(pub_key_cred_params.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        timeout: ::core::convert::TryInto::try_into(value.timeout()?)?,
                        attestation: ::core::convert::TryInto::try_into(value.attestation()?)?,
                        exclude_credentials: if let ::core::option::Option::Some(
                            exclude_credentials,
                        ) = value.exclude_credentials()?
                        {
                            ::core::option::Option::Some(exclude_credentials.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                        authenticator_selection: if let ::core::option::Option::Some(
                            authenticator_selection,
                        ) = value.authenticator_selection()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(authenticator_selection)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        extensions: if let ::core::option::Option::Some(extensions) =
                            value.extensions()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(extensions)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for CredentialCreationOptionsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for CredentialCreationOptionsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[CredentialCreationOptionsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<CredentialCreationOptions>>
                for CredentialCreationOptions
            {
                type Value = ::planus::Offset<CredentialCreationOptions>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<CredentialCreationOptions>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for CredentialCreationOptionsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[CredentialCreationOptionsRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The enum `AuthenticatorTransport` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Enum `AuthenticatorTransport` in the file `auth/webauthn.fbs:38`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i8)]
            pub enum AuthenticatorTransport {
                ///  <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-usb>
                Usb = 0,

                ///  <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-nfc>
                Nfc = 1,

                ///  <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-ble>
                Ble = 2,

                ///  <https://www.w3.org/TR/webauthn/#dom-authenticatortransport-internal>
                Internal = 3,

                ///  Hybrid transport, formerly caBLE. Part of the level 3 draft specification.
                ///  <https://w3c.github.io/webauthn/#dom-authenticatortransport-hybrid>
                Hybrid = 4,

                ///  Test transport; used for Windows 10.
                Test = 5,
            }

            impl AuthenticatorTransport {
                /// Array containing all valid variants of AuthenticatorTransport
                pub const ENUM_VALUES: [Self; 6] = [
                    Self::Usb,
                    Self::Nfc,
                    Self::Ble,
                    Self::Internal,
                    Self::Hybrid,
                    Self::Test,
                ];
            }

            impl ::core::convert::TryFrom<i8> for AuthenticatorTransport {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(AuthenticatorTransport::Usb),
                        1 => ::core::result::Result::Ok(AuthenticatorTransport::Nfc),
                        2 => ::core::result::Result::Ok(AuthenticatorTransport::Ble),
                        3 => ::core::result::Result::Ok(AuthenticatorTransport::Internal),
                        4 => ::core::result::Result::Ok(AuthenticatorTransport::Hybrid),
                        5 => ::core::result::Result::Ok(AuthenticatorTransport::Test),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<AuthenticatorTransport> for i8 {
                #[inline]
                fn from(value: AuthenticatorTransport) -> Self {
                    value as i8
                }
            }

            impl ::planus::Primitive for AuthenticatorTransport {
                const ALIGNMENT: usize = 1;
                const SIZE: usize = 1;
            }

            impl ::planus::WriteAsPrimitive<AuthenticatorTransport> for AuthenticatorTransport {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i8).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<AuthenticatorTransport> for AuthenticatorTransport {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> AuthenticatorTransport {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<AuthenticatorTransport, AuthenticatorTransport>
                for AuthenticatorTransport
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &AuthenticatorTransport,
                ) -> ::core::option::Option<AuthenticatorTransport> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<AuthenticatorTransport> for AuthenticatorTransport {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<AuthenticatorTransport> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for AuthenticatorTransport {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for AuthenticatorTransport {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 1;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value = *buffer.buffer.get_unchecked(offset) as i8;
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "AuthenticatorTransport",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<AuthenticatorTransport> for AuthenticatorTransport {
                const STRIDE: usize = 1;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - i as u32,
                        );
                    }
                }
            }

            /// The table `AuthenticatorAttestationResponse` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `AuthenticatorAttestationResponse` in the file `auth/webauthn.fbs:54`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct AuthenticatorAttestationResponse {
                /// The field `client_data` in the table `AuthenticatorAttestationResponse`
                pub client_data: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `attestation_object` in the table `AuthenticatorAttestationResponse`
                pub attestation_object: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `transports` in the table `AuthenticatorAttestationResponse`
                pub transports:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::AuthenticatorTransport>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for AuthenticatorAttestationResponse {
                fn default() -> Self {
                    Self {
                        client_data: ::core::default::Default::default(),
                        attestation_object: ::core::default::Default::default(),
                        transports: ::core::default::Default::default(),
                    }
                }
            }

            impl AuthenticatorAttestationResponse {
                /// Creates a [AuthenticatorAttestationResponseBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> AuthenticatorAttestationResponseBuilder<()> {
                    AuthenticatorAttestationResponseBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_client_data: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_attestation_object: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_transports: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[self::AuthenticatorTransport]>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_client_data = field_client_data.prepare(builder);
                    let prepared_attestation_object = field_attestation_object.prepare(builder);
                    let prepared_transports = field_transports.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_client_data.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(0);
                    }
                    if prepared_attestation_object.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(1);
                    }
                    if prepared_transports.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[self::AuthenticatorTransport]>>(2);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_client_data) =
                                prepared_client_data
                            {
                                object_writer.write::<_, _, 4>(&prepared_client_data);
                            }
                            if let ::core::option::Option::Some(prepared_attestation_object) =
                                prepared_attestation_object
                            {
                                object_writer.write::<_, _, 4>(&prepared_attestation_object);
                            }
                            if let ::core::option::Option::Some(prepared_transports) =
                                prepared_transports
                            {
                                object_writer.write::<_, _, 4>(&prepared_transports);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponse
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorAttestationResponse> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponse
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<AuthenticatorAttestationResponse>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<AuthenticatorAttestationResponse>
                for AuthenticatorAttestationResponse
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorAttestationResponse> {
                    AuthenticatorAttestationResponse::create(
                        builder,
                        &self.client_data,
                        &self.attestation_object,
                        &self.transports,
                    )
                }
            }

            /// Builder for serializing an instance of the [AuthenticatorAttestationResponse] type.
            ///
            /// Can be created using the [AuthenticatorAttestationResponse::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct AuthenticatorAttestationResponseBuilder<State>(State);

            impl AuthenticatorAttestationResponseBuilder<()> {
                /// Setter for the [`client_data` field](AuthenticatorAttestationResponse#structfield.client_data).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn client_data<T0>(
                    self,
                    value: T0,
                ) -> AuthenticatorAttestationResponseBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    AuthenticatorAttestationResponseBuilder((value,))
                }

                /// Sets the [`client_data` field](AuthenticatorAttestationResponse#structfield.client_data) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn client_data_as_null(self) -> AuthenticatorAttestationResponseBuilder<((),)> {
                    self.client_data(())
                }
            }

            impl<T0> AuthenticatorAttestationResponseBuilder<(T0,)> {
                /// Setter for the [`attestation_object` field](AuthenticatorAttestationResponse#structfield.attestation_object).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn attestation_object<T1>(
                    self,
                    value: T1,
                ) -> AuthenticatorAttestationResponseBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0,) = self.0;
                    AuthenticatorAttestationResponseBuilder((v0, value))
                }

                /// Sets the [`attestation_object` field](AuthenticatorAttestationResponse#structfield.attestation_object) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn attestation_object_as_null(
                    self,
                ) -> AuthenticatorAttestationResponseBuilder<(T0, ())> {
                    self.attestation_object(())
                }
            }

            impl<T0, T1> AuthenticatorAttestationResponseBuilder<(T0, T1)> {
                /// Setter for the [`transports` field](AuthenticatorAttestationResponse#structfield.transports).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn transports<T2>(
                    self,
                    value: T2,
                ) -> AuthenticatorAttestationResponseBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                {
                    let (v0, v1) = self.0;
                    AuthenticatorAttestationResponseBuilder((v0, v1, value))
                }

                /// Sets the [`transports` field](AuthenticatorAttestationResponse#structfield.transports) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn transports_as_null(
                    self,
                ) -> AuthenticatorAttestationResponseBuilder<(T0, T1, ())> {
                    self.transports(())
                }
            }

            impl<T0, T1, T2> AuthenticatorAttestationResponseBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [AuthenticatorAttestationResponse].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorAttestationResponse>
                where
                    Self: ::planus::WriteAsOffset<AuthenticatorAttestationResponse>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                > ::planus::WriteAs<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponseBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<AuthenticatorAttestationResponse>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorAttestationResponse> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                >
                ::planus::WriteAsOptional<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponseBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<AuthenticatorAttestationResponse>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<AuthenticatorAttestationResponse>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                > ::planus::WriteAsOffset<AuthenticatorAttestationResponse>
                for AuthenticatorAttestationResponseBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorAttestationResponse> {
                    let (v0, v1, v2) = &self.0;
                    AuthenticatorAttestationResponse::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [AuthenticatorAttestationResponse].
            #[derive(Copy, Clone)]
            pub struct AuthenticatorAttestationResponseRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> AuthenticatorAttestationResponseRef<'a> {
                /// Getter for the [`client_data` field](AuthenticatorAttestationResponse#structfield.client_data).
                #[inline]
                pub fn client_data(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0
                        .access(0, "AuthenticatorAttestationResponse", "client_data")
                }

                /// Getter for the [`attestation_object` field](AuthenticatorAttestationResponse#structfield.attestation_object).
                #[inline]
                pub fn attestation_object(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0
                        .access(1, "AuthenticatorAttestationResponse", "attestation_object")
                }

                /// Getter for the [`transports` field](AuthenticatorAttestationResponse#structfield.transports).
                #[inline]
                pub fn transports(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<
                            'a,
                            ::core::result::Result<
                                self::AuthenticatorTransport,
                                ::planus::errors::UnknownEnumTag,
                            >,
                        >,
                    >,
                > {
                    self.0
                        .access(2, "AuthenticatorAttestationResponse", "transports")
                }
            }

            impl<'a> ::core::fmt::Debug for AuthenticatorAttestationResponseRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("AuthenticatorAttestationResponseRef");
                    if let ::core::option::Option::Some(field_client_data) =
                        self.client_data().transpose()
                    {
                        f.field("client_data", &field_client_data);
                    }
                    if let ::core::option::Option::Some(field_attestation_object) =
                        self.attestation_object().transpose()
                    {
                        f.field("attestation_object", &field_attestation_object);
                    }
                    if let ::core::option::Option::Some(field_transports) =
                        self.transports().transpose()
                    {
                        f.field("transports", &field_transports);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<AuthenticatorAttestationResponseRef<'a>>
                for AuthenticatorAttestationResponse
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(
                    value: AuthenticatorAttestationResponseRef<'a>,
                ) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        client_data: value.client_data()?.map(|v| v.to_vec()),
                        attestation_object: value.attestation_object()?.map(|v| v.to_vec()),
                        transports: if let ::core::option::Option::Some(transports) =
                            value.transports()?
                        {
                            ::core::option::Option::Some(transports.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for AuthenticatorAttestationResponseRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for AuthenticatorAttestationResponseRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[AuthenticatorAttestationResponseRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponse
            {
                type Value = ::planus::Offset<AuthenticatorAttestationResponse>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<AuthenticatorAttestationResponse>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for AuthenticatorAttestationResponseRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[AuthenticatorAttestationResponseRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The enum `CredentialProtectionPolicy` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Enum `CredentialProtectionPolicy` in the file `auth/webauthn.fbs:60`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i8)]
            pub enum CredentialProtectionPolicy {
                ///  This reflects "FIDO_2_0" semantics. In this configuration, performing
                ///  some form of user verification is optional with or without credentialID
                ///  list. This is the default state of the credential if the extension is
                ///  not specified.
                UserVerificationOptional = 0,

                ///  In this configuration, credential is discovered only when its
                ///  credentialID is provided by the platform or when some form of user
                ///  verification is performed.
                UserVerificationOptionalWithCredentialIdList = 1,

                ///  This reflects that discovery and usage of the credential MUST be
                ///  preceded by some form of user verification.
                UserVerificationRequired = 2,
            }

            impl CredentialProtectionPolicy {
                /// Array containing all valid variants of CredentialProtectionPolicy
                pub const ENUM_VALUES: [Self; 3] = [
                    Self::UserVerificationOptional,
                    Self::UserVerificationOptionalWithCredentialIdList,
                    Self::UserVerificationRequired,
                ];
            }

            impl ::core::convert::TryFrom<i8> for CredentialProtectionPolicy {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationOptional),
                        1 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationOptionalWithCredentialIdList),
                        2 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationRequired),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind { tag: value as i128 }),
                    }
                }
            }

            impl ::core::convert::From<CredentialProtectionPolicy> for i8 {
                #[inline]
                fn from(value: CredentialProtectionPolicy) -> Self {
                    value as i8
                }
            }

            impl ::planus::Primitive for CredentialProtectionPolicy {
                const ALIGNMENT: usize = 1;
                const SIZE: usize = 1;
            }

            impl ::planus::WriteAsPrimitive<CredentialProtectionPolicy> for CredentialProtectionPolicy {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i8).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<CredentialProtectionPolicy> for CredentialProtectionPolicy {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> CredentialProtectionPolicy {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<CredentialProtectionPolicy, CredentialProtectionPolicy>
                for CredentialProtectionPolicy
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &CredentialProtectionPolicy,
                ) -> ::core::option::Option<CredentialProtectionPolicy> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<CredentialProtectionPolicy> for CredentialProtectionPolicy {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<CredentialProtectionPolicy> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for CredentialProtectionPolicy {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for CredentialProtectionPolicy {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 1;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value = *buffer.buffer.get_unchecked(offset) as i8;
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "CredentialProtectionPolicy",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<CredentialProtectionPolicy> for CredentialProtectionPolicy {
                const STRIDE: usize = 1;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - i as u32,
                        );
                    }
                }
            }

            /// The table `RegistrationExtensionsClientOutputs` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `RegistrationExtensionsClientOutputs` in the file `auth/webauthn.fbs:75`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct RegistrationExtensionsClientOutputs {
                /// The field `app_id` in the table `RegistrationExtensionsClientOutputs`
                pub app_id: bool,
                /// The field `cred_props` in the table `RegistrationExtensionsClientOutputs`
                pub cred_props: bool,
                /// The field `hmac_secret` in the table `RegistrationExtensionsClientOutputs`
                pub hmac_secret: bool,
                /// The field `cred_protect` in the table `RegistrationExtensionsClientOutputs`
                pub cred_protect: self::CredentialProtectionPolicy,
                /// The field `min_pin_length` in the table `RegistrationExtensionsClientOutputs`
                pub min_pin_length: u32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for RegistrationExtensionsClientOutputs {
                fn default() -> Self {
                    Self {
                        app_id: false,
                        cred_props: false,
                        hmac_secret: false,
                        cred_protect: self::CredentialProtectionPolicy::UserVerificationOptional,
                        min_pin_length: 0,
                    }
                }
            }

            impl RegistrationExtensionsClientOutputs {
                /// Creates a [RegistrationExtensionsClientOutputsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> RegistrationExtensionsClientOutputsBuilder<()> {
                    RegistrationExtensionsClientOutputsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_app_id: impl ::planus::WriteAsDefault<bool, bool>,
                    field_cred_props: impl ::planus::WriteAsDefault<bool, bool>,
                    field_hmac_secret: impl ::planus::WriteAsDefault<bool, bool>,
                    field_cred_protect: impl ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    field_min_pin_length: impl ::planus::WriteAsDefault<u32, u32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_app_id = field_app_id.prepare(builder, &false);
                    let prepared_cred_props = field_cred_props.prepare(builder, &false);
                    let prepared_hmac_secret = field_hmac_secret.prepare(builder, &false);
                    let prepared_cred_protect = field_cred_protect.prepare(
                        builder,
                        &self::CredentialProtectionPolicy::UserVerificationOptional,
                    );
                    let prepared_min_pin_length = field_min_pin_length.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<14> =
                        ::core::default::Default::default();
                    if prepared_min_pin_length.is_some() {
                        table_writer.write_entry::<u32>(4);
                    }
                    if prepared_app_id.is_some() {
                        table_writer.write_entry::<bool>(0);
                    }
                    if prepared_cred_props.is_some() {
                        table_writer.write_entry::<bool>(1);
                    }
                    if prepared_hmac_secret.is_some() {
                        table_writer.write_entry::<bool>(2);
                    }
                    if prepared_cred_protect.is_some() {
                        table_writer.write_entry::<self::CredentialProtectionPolicy>(3);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_min_pin_length) =
                                prepared_min_pin_length
                            {
                                object_writer.write::<_, _, 4>(&prepared_min_pin_length);
                            }
                            if let ::core::option::Option::Some(prepared_app_id) = prepared_app_id {
                                object_writer.write::<_, _, 1>(&prepared_app_id);
                            }
                            if let ::core::option::Option::Some(prepared_cred_props) =
                                prepared_cred_props
                            {
                                object_writer.write::<_, _, 1>(&prepared_cred_props);
                            }
                            if let ::core::option::Option::Some(prepared_hmac_secret) =
                                prepared_hmac_secret
                            {
                                object_writer.write::<_, _, 1>(&prepared_hmac_secret);
                            }
                            if let ::core::option::Option::Some(prepared_cred_protect) =
                                prepared_cred_protect
                            {
                                object_writer.write::<_, _, 1>(&prepared_cred_protect);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputs
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputs
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RegistrationExtensionsClientOutputs>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RegistrationExtensionsClientOutputs>
                for RegistrationExtensionsClientOutputs
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs> {
                    RegistrationExtensionsClientOutputs::create(
                        builder,
                        self.app_id,
                        self.cred_props,
                        self.hmac_secret,
                        self.cred_protect,
                        self.min_pin_length,
                    )
                }
            }

            /// Builder for serializing an instance of the [RegistrationExtensionsClientOutputs] type.
            ///
            /// Can be created using the [RegistrationExtensionsClientOutputs::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct RegistrationExtensionsClientOutputsBuilder<State>(State);

            impl RegistrationExtensionsClientOutputsBuilder<()> {
                /// Setter for the [`app_id` field](RegistrationExtensionsClientOutputs#structfield.app_id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn app_id<T0>(
                    self,
                    value: T0,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<bool, bool>,
                {
                    RegistrationExtensionsClientOutputsBuilder((value,))
                }

                /// Sets the [`app_id` field](RegistrationExtensionsClientOutputs#structfield.app_id) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn app_id_as_default(
                    self,
                ) -> RegistrationExtensionsClientOutputsBuilder<(::planus::DefaultValue,)>
                {
                    self.app_id(::planus::DefaultValue)
                }
            }

            impl<T0> RegistrationExtensionsClientOutputsBuilder<(T0,)> {
                /// Setter for the [`cred_props` field](RegistrationExtensionsClientOutputs#structfield.cred_props).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_props<T1>(
                    self,
                    value: T1,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0,) = self.0;
                    RegistrationExtensionsClientOutputsBuilder((v0, value))
                }

                /// Sets the [`cred_props` field](RegistrationExtensionsClientOutputs#structfield.cred_props) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_props_as_default(
                    self,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, ::planus::DefaultValue)>
                {
                    self.cred_props(::planus::DefaultValue)
                }
            }

            impl<T0, T1> RegistrationExtensionsClientOutputsBuilder<(T0, T1)> {
                /// Setter for the [`hmac_secret` field](RegistrationExtensionsClientOutputs#structfield.hmac_secret).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hmac_secret<T2>(
                    self,
                    value: T2,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1) = self.0;
                    RegistrationExtensionsClientOutputsBuilder((v0, v1, value))
                }

                /// Sets the [`hmac_secret` field](RegistrationExtensionsClientOutputs#structfield.hmac_secret) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hmac_secret_as_default(
                    self,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1, ::planus::DefaultValue)>
                {
                    self.hmac_secret(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2)> {
                /// Setter for the [`cred_protect` field](RegistrationExtensionsClientOutputs#structfield.cred_protect).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_protect<T3>(
                    self,
                    value: T3,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                {
                    let (v0, v1, v2) = self.0;
                    RegistrationExtensionsClientOutputsBuilder((v0, v1, v2, value))
                }

                /// Sets the [`cred_protect` field](RegistrationExtensionsClientOutputs#structfield.cred_protect) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_protect_as_default(
                    self,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, ::planus::DefaultValue)>
                {
                    self.cred_protect(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`min_pin_length` field](RegistrationExtensionsClientOutputs#structfield.min_pin_length).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_pin_length<T4>(
                    self,
                    value: T4,
                ) -> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsDefault<u32, u32>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    RegistrationExtensionsClientOutputsBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`min_pin_length` field](RegistrationExtensionsClientOutputs#structfield.min_pin_length) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_pin_length_as_default(
                    self,
                ) -> RegistrationExtensionsClientOutputsBuilder<(
                    T0,
                    T1,
                    T2,
                    T3,
                    ::planus::DefaultValue,
                )> {
                    self.min_pin_length(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4> RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3, T4)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RegistrationExtensionsClientOutputs].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs>
                where
                    Self: ::planus::WriteAsOffset<RegistrationExtensionsClientOutputs>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<bool, bool>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                >
                ::planus::WriteAs<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3, T4)>
            {
                type Prepared = ::planus::Offset<RegistrationExtensionsClientOutputs>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<bool, bool>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                >
                ::planus::WriteAsOptional<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3, T4)>
            {
                type Prepared = ::planus::Offset<RegistrationExtensionsClientOutputs>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RegistrationExtensionsClientOutputs>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<bool, bool>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T4: ::planus::WriteAsDefault<u32, u32>,
                > ::planus::WriteAsOffset<RegistrationExtensionsClientOutputs>
                for RegistrationExtensionsClientOutputsBuilder<(T0, T1, T2, T3, T4)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs> {
                    let (v0, v1, v2, v3, v4) = &self.0;
                    RegistrationExtensionsClientOutputs::create(builder, v0, v1, v2, v3, v4)
                }
            }

            /// Reference to a deserialized [RegistrationExtensionsClientOutputs].
            #[derive(Copy, Clone)]
            pub struct RegistrationExtensionsClientOutputsRef<'a>(
                ::planus::table_reader::Table<'a>,
            );

            impl<'a> RegistrationExtensionsClientOutputsRef<'a> {
                /// Getter for the [`app_id` field](RegistrationExtensionsClientOutputs#structfield.app_id).
                #[inline]
                pub fn app_id(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "RegistrationExtensionsClientOutputs", "app_id")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`cred_props` field](RegistrationExtensionsClientOutputs#structfield.cred_props).
                #[inline]
                pub fn cred_props(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "RegistrationExtensionsClientOutputs", "cred_props")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`hmac_secret` field](RegistrationExtensionsClientOutputs#structfield.hmac_secret).
                #[inline]
                pub fn hmac_secret(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "RegistrationExtensionsClientOutputs", "hmac_secret")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`cred_protect` field](RegistrationExtensionsClientOutputs#structfield.cred_protect).
                #[inline]
                pub fn cred_protect(&self) -> ::planus::Result<self::CredentialProtectionPolicy> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(3, "RegistrationExtensionsClientOutputs", "cred_protect")?
                            .unwrap_or(self::CredentialProtectionPolicy::UserVerificationOptional),
                    )
                }

                /// Getter for the [`min_pin_length` field](RegistrationExtensionsClientOutputs#structfield.min_pin_length).
                #[inline]
                pub fn min_pin_length(&self) -> ::planus::Result<u32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(4, "RegistrationExtensionsClientOutputs", "min_pin_length")?
                            .unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for RegistrationExtensionsClientOutputsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("RegistrationExtensionsClientOutputsRef");
                    f.field("app_id", &self.app_id());
                    f.field("cred_props", &self.cred_props());
                    f.field("hmac_secret", &self.hmac_secret());
                    f.field("cred_protect", &self.cred_protect());
                    f.field("min_pin_length", &self.min_pin_length());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<RegistrationExtensionsClientOutputsRef<'a>>
                for RegistrationExtensionsClientOutputs
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(
                    value: RegistrationExtensionsClientOutputsRef<'a>,
                ) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        app_id: ::core::convert::TryInto::try_into(value.app_id()?)?,
                        cred_props: ::core::convert::TryInto::try_into(value.cred_props()?)?,
                        hmac_secret: ::core::convert::TryInto::try_into(value.hmac_secret()?)?,
                        cred_protect: ::core::convert::TryInto::try_into(value.cred_protect()?)?,
                        min_pin_length: ::core::convert::TryInto::try_into(
                            value.min_pin_length()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for RegistrationExtensionsClientOutputsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for RegistrationExtensionsClientOutputsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RegistrationExtensionsClientOutputsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputs
            {
                type Value = ::planus::Offset<RegistrationExtensionsClientOutputs>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<RegistrationExtensionsClientOutputs>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for RegistrationExtensionsClientOutputsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RegistrationExtensionsClientOutputsRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The table `RegisterPublicKeyCredential` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `RegisterPublicKeyCredential` in the file `auth/webauthn.fbs:83`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct RegisterPublicKeyCredential {
                /// The field `id` in the table `RegisterPublicKeyCredential`
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `raw_id` in the table `RegisterPublicKeyCredential`
                pub raw_id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `response` in the table `RegisterPublicKeyCredential`
                pub response: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::AuthenticatorAttestationResponse>,
                >,
                /// The field `client_extension_results` in the table `RegisterPublicKeyCredential`
                pub client_extension_results: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::RegistrationExtensionsClientOutputs>,
                >,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for RegisterPublicKeyCredential {
                fn default() -> Self {
                    Self {
                        id: ::core::default::Default::default(),
                        raw_id: ::core::default::Default::default(),
                        response: ::core::default::Default::default(),
                        client_extension_results: ::core::default::Default::default(),
                    }
                }
            }

            impl RegisterPublicKeyCredential {
                /// Creates a [RegisterPublicKeyCredentialBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> RegisterPublicKeyCredentialBuilder<()> {
                    RegisterPublicKeyCredentialBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_id: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_raw_id: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_response: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorAttestationResponse>,
                    >,
                    field_client_extension_results: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::RegistrationExtensionsClientOutputs>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_id = field_id.prepare(builder);
                    let prepared_raw_id = field_raw_id.prepare(builder);
                    let prepared_response = field_response.prepare(builder);
                    let prepared_client_extension_results =
                        field_client_extension_results.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<12> =
                        ::core::default::Default::default();
                    if prepared_id.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(0);
                    }
                    if prepared_raw_id.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(1);
                    }
                    if prepared_response.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::AuthenticatorAttestationResponse>>(2);
                    }
                    if prepared_client_extension_results.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::RegistrationExtensionsClientOutputs>>(3);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_id) = prepared_id {
                                object_writer.write::<_, _, 4>(&prepared_id);
                            }
                            if let ::core::option::Option::Some(prepared_raw_id) = prepared_raw_id {
                                object_writer.write::<_, _, 4>(&prepared_raw_id);
                            }
                            if let ::core::option::Option::Some(prepared_response) =
                                prepared_response
                            {
                                object_writer.write::<_, _, 4>(&prepared_response);
                            }
                            if let ::core::option::Option::Some(prepared_client_extension_results) =
                                prepared_client_extension_results
                            {
                                object_writer.write::<_, _, 4>(&prepared_client_extension_results);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredential
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegisterPublicKeyCredential> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredential
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RegisterPublicKeyCredential>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RegisterPublicKeyCredential> for RegisterPublicKeyCredential {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegisterPublicKeyCredential> {
                    RegisterPublicKeyCredential::create(
                        builder,
                        &self.id,
                        &self.raw_id,
                        &self.response,
                        &self.client_extension_results,
                    )
                }
            }

            /// Builder for serializing an instance of the [RegisterPublicKeyCredential] type.
            ///
            /// Can be created using the [RegisterPublicKeyCredential::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct RegisterPublicKeyCredentialBuilder<State>(State);

            impl RegisterPublicKeyCredentialBuilder<()> {
                /// Setter for the [`id` field](RegisterPublicKeyCredential#structfield.id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id<T0>(self, value: T0) -> RegisterPublicKeyCredentialBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    RegisterPublicKeyCredentialBuilder((value,))
                }

                /// Sets the [`id` field](RegisterPublicKeyCredential#structfield.id) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id_as_null(self) -> RegisterPublicKeyCredentialBuilder<((),)> {
                    self.id(())
                }
            }

            impl<T0> RegisterPublicKeyCredentialBuilder<(T0,)> {
                /// Setter for the [`raw_id` field](RegisterPublicKeyCredential#structfield.raw_id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn raw_id<T1>(self, value: T1) -> RegisterPublicKeyCredentialBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0,) = self.0;
                    RegisterPublicKeyCredentialBuilder((v0, value))
                }

                /// Sets the [`raw_id` field](RegisterPublicKeyCredential#structfield.raw_id) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn raw_id_as_null(self) -> RegisterPublicKeyCredentialBuilder<(T0, ())> {
                    self.raw_id(())
                }
            }

            impl<T0, T1> RegisterPublicKeyCredentialBuilder<(T0, T1)> {
                /// Setter for the [`response` field](RegisterPublicKeyCredential#structfield.response).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn response<T2>(
                    self,
                    value: T2,
                ) -> RegisterPublicKeyCredentialBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorAttestationResponse>,
                    >,
                {
                    let (v0, v1) = self.0;
                    RegisterPublicKeyCredentialBuilder((v0, v1, value))
                }

                /// Sets the [`response` field](RegisterPublicKeyCredential#structfield.response) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn response_as_null(self) -> RegisterPublicKeyCredentialBuilder<(T0, T1, ())> {
                    self.response(())
                }
            }

            impl<T0, T1, T2> RegisterPublicKeyCredentialBuilder<(T0, T1, T2)> {
                /// Setter for the [`client_extension_results` field](RegisterPublicKeyCredential#structfield.client_extension_results).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn client_extension_results<T3>(
                    self,
                    value: T3,
                ) -> RegisterPublicKeyCredentialBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RegistrationExtensionsClientOutputs>,
                    >,
                {
                    let (v0, v1, v2) = self.0;
                    RegisterPublicKeyCredentialBuilder((v0, v1, v2, value))
                }

                /// Sets the [`client_extension_results` field](RegisterPublicKeyCredential#structfield.client_extension_results) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn client_extension_results_as_null(
                    self,
                ) -> RegisterPublicKeyCredentialBuilder<(T0, T1, T2, ())> {
                    self.client_extension_results(())
                }
            }

            impl<T0, T1, T2, T3> RegisterPublicKeyCredentialBuilder<(T0, T1, T2, T3)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RegisterPublicKeyCredential].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegisterPublicKeyCredential>
                where
                    Self: ::planus::WriteAsOffset<RegisterPublicKeyCredential>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorAttestationResponse>,
                    >,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RegistrationExtensionsClientOutputs>,
                    >,
                > ::planus::WriteAs<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredentialBuilder<(T0, T1, T2, T3)>
            {
                type Prepared = ::planus::Offset<RegisterPublicKeyCredential>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegisterPublicKeyCredential> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorAttestationResponse>,
                    >,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RegistrationExtensionsClientOutputs>,
                    >,
                >
                ::planus::WriteAsOptional<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredentialBuilder<(T0, T1, T2, T3)>
            {
                type Prepared = ::planus::Offset<RegisterPublicKeyCredential>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RegisterPublicKeyCredential>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<
                        ::planus::Offset<self::AuthenticatorAttestationResponse>,
                    >,
                    T3: ::planus::WriteAsOptional<
                        ::planus::Offset<self::RegistrationExtensionsClientOutputs>,
                    >,
                > ::planus::WriteAsOffset<RegisterPublicKeyCredential>
                for RegisterPublicKeyCredentialBuilder<(T0, T1, T2, T3)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegisterPublicKeyCredential> {
                    let (v0, v1, v2, v3) = &self.0;
                    RegisterPublicKeyCredential::create(builder, v0, v1, v2, v3)
                }
            }

            /// Reference to a deserialized [RegisterPublicKeyCredential].
            #[derive(Copy, Clone)]
            pub struct RegisterPublicKeyCredentialRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RegisterPublicKeyCredentialRef<'a> {
                /// Getter for the [`id` field](RegisterPublicKeyCredential#structfield.id).
                #[inline]
                pub fn id(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(0, "RegisterPublicKeyCredential", "id")
                }

                /// Getter for the [`raw_id` field](RegisterPublicKeyCredential#structfield.raw_id).
                #[inline]
                pub fn raw_id(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(1, "RegisterPublicKeyCredential", "raw_id")
                }

                /// Getter for the [`response` field](RegisterPublicKeyCredential#structfield.response).
                #[inline]
                pub fn response(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::AuthenticatorAttestationResponseRef<'a>>,
                > {
                    self.0.access(2, "RegisterPublicKeyCredential", "response")
                }

                /// Getter for the [`client_extension_results` field](RegisterPublicKeyCredential#structfield.client_extension_results).
                #[inline]
                pub fn client_extension_results(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::RegistrationExtensionsClientOutputsRef<'a>>,
                > {
                    self.0
                        .access(3, "RegisterPublicKeyCredential", "client_extension_results")
                }
            }

            impl<'a> ::core::fmt::Debug for RegisterPublicKeyCredentialRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("RegisterPublicKeyCredentialRef");
                    if let ::core::option::Option::Some(field_id) = self.id().transpose() {
                        f.field("id", &field_id);
                    }
                    if let ::core::option::Option::Some(field_raw_id) = self.raw_id().transpose() {
                        f.field("raw_id", &field_raw_id);
                    }
                    if let ::core::option::Option::Some(field_response) =
                        self.response().transpose()
                    {
                        f.field("response", &field_response);
                    }
                    if let ::core::option::Option::Some(field_client_extension_results) =
                        self.client_extension_results().transpose()
                    {
                        f.field("client_extension_results", &field_client_extension_results);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<RegisterPublicKeyCredentialRef<'a>>
                for RegisterPublicKeyCredential
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: RegisterPublicKeyCredentialRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        id: value.id()?.map(|v| v.to_vec()),
                        raw_id: value.raw_id()?.map(|v| v.to_vec()),
                        response: if let ::core::option::Option::Some(response) =
                            value.response()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(response)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        client_extension_results: if let ::core::option::Option::Some(
                            client_extension_results,
                        ) = value.client_extension_results()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(client_extension_results)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for RegisterPublicKeyCredentialRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for RegisterPublicKeyCredentialRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RegisterPublicKeyCredentialRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredential
            {
                type Value = ::planus::Offset<RegisterPublicKeyCredential>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<RegisterPublicKeyCredential>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for RegisterPublicKeyCredentialRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RegisterPublicKeyCredentialRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The table `CredProtect` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `CredProtect` in the file `auth/webauthn.fbs:90`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct CredProtect {
                ///  The credential policy to enact
                pub credential_protection_policy: self::CredentialProtectionPolicy,
                ///  Whether it is better for the authenticator to fail to create a
                ///  credential rather than ignore the protection policy
                pub enforce_credential_protection_policy: bool,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for CredProtect {
                fn default() -> Self {
                    Self {
                        credential_protection_policy:
                            self::CredentialProtectionPolicy::UserVerificationOptional,
                        enforce_credential_protection_policy: false,
                    }
                }
            }

            impl CredProtect {
                /// Creates a [CredProtectBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> CredProtectBuilder<()> {
                    CredProtectBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_credential_protection_policy: impl ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    field_enforce_credential_protection_policy: impl ::planus::WriteAsDefault<
                        bool,
                        bool,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_credential_protection_policy = field_credential_protection_policy
                        .prepare(
                            builder,
                            &self::CredentialProtectionPolicy::UserVerificationOptional,
                        );
                    let prepared_enforce_credential_protection_policy =
                        field_enforce_credential_protection_policy.prepare(builder, &false);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_credential_protection_policy.is_some() {
                        table_writer.write_entry::<self::CredentialProtectionPolicy>(0);
                    }
                    if prepared_enforce_credential_protection_policy.is_some() {
                        table_writer.write_entry::<bool>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(
                                prepared_credential_protection_policy,
                            ) = prepared_credential_protection_policy
                            {
                                object_writer
                                    .write::<_, _, 1>(&prepared_credential_protection_policy);
                            }
                            if let ::core::option::Option::Some(
                                prepared_enforce_credential_protection_policy,
                            ) = prepared_enforce_credential_protection_policy
                            {
                                object_writer.write::<_, _, 1>(
                                    &prepared_enforce_credential_protection_policy,
                                );
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<CredProtect>> for CredProtect {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<CredProtect>> for CredProtect {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredProtect>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<CredProtect> for CredProtect {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    CredProtect::create(
                        builder,
                        self.credential_protection_policy,
                        self.enforce_credential_protection_policy,
                    )
                }
            }

            /// Builder for serializing an instance of the [CredProtect] type.
            ///
            /// Can be created using the [CredProtect::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct CredProtectBuilder<State>(State);

            impl CredProtectBuilder<()> {
                /// Setter for the [`credential_protection_policy` field](CredProtect#structfield.credential_protection_policy).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn credential_protection_policy<T0>(
                    self,
                    value: T0,
                ) -> CredProtectBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                {
                    CredProtectBuilder((value,))
                }

                /// Sets the [`credential_protection_policy` field](CredProtect#structfield.credential_protection_policy) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn credential_protection_policy_as_default(
                    self,
                ) -> CredProtectBuilder<(::planus::DefaultValue,)> {
                    self.credential_protection_policy(::planus::DefaultValue)
                }
            }

            impl<T0> CredProtectBuilder<(T0,)> {
                /// Setter for the [`enforce_credential_protection_policy` field](CredProtect#structfield.enforce_credential_protection_policy).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn enforce_credential_protection_policy<T1>(
                    self,
                    value: T1,
                ) -> CredProtectBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0,) = self.0;
                    CredProtectBuilder((v0, value))
                }

                /// Sets the [`enforce_credential_protection_policy` field](CredProtect#structfield.enforce_credential_protection_policy) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn enforce_credential_protection_policy_as_default(
                    self,
                ) -> CredProtectBuilder<(T0, ::planus::DefaultValue)> {
                    self.enforce_credential_protection_policy(::planus::DefaultValue)
                }
            }

            impl<T0, T1> CredProtectBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [CredProtect].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect>
                where
                    Self: ::planus::WriteAsOffset<CredProtect>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAs<::planus::Offset<CredProtect>>
                for CredProtectBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<CredProtect>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAsOptional<::planus::Offset<CredProtect>>
                for CredProtectBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<CredProtect>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredProtect>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::CredentialProtectionPolicy,
                        self::CredentialProtectionPolicy,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAsOffset<CredProtect> for CredProtectBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    let (v0, v1) = &self.0;
                    CredProtect::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [CredProtect].
            #[derive(Copy, Clone)]
            pub struct CredProtectRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> CredProtectRef<'a> {
                /// Getter for the [`credential_protection_policy` field](CredProtect#structfield.credential_protection_policy).
                #[inline]
                pub fn credential_protection_policy(
                    &self,
                ) -> ::planus::Result<self::CredentialProtectionPolicy> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "CredProtect", "credential_protection_policy")?
                            .unwrap_or(self::CredentialProtectionPolicy::UserVerificationOptional),
                    )
                }

                /// Getter for the [`enforce_credential_protection_policy` field](CredProtect#structfield.enforce_credential_protection_policy).
                #[inline]
                pub fn enforce_credential_protection_policy(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "CredProtect", "enforce_credential_protection_policy")?
                            .unwrap_or(false),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for CredProtectRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("CredProtectRef");
                    f.field(
                        "credential_protection_policy",
                        &self.credential_protection_policy(),
                    );
                    f.field(
                        "enforce_credential_protection_policy",
                        &self.enforce_credential_protection_policy(),
                    );
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<CredProtectRef<'a>> for CredProtect {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: CredProtectRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        credential_protection_policy: ::core::convert::TryInto::try_into(
                            value.credential_protection_policy()?,
                        )?,
                        enforce_credential_protection_policy: ::core::convert::TryInto::try_into(
                            value.enforce_credential_protection_policy()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for CredProtectRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for CredProtectRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[CredProtectRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<CredProtect>> for CredProtect {
                type Value = ::planus::Offset<CredProtect>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<CredProtect>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for CredProtectRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[CredProtectRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `RequestRegistrationExtensions` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `RequestRegistrationExtensions` in the file `auth/webauthn.fbs:98`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct RequestRegistrationExtensions {
                ///  The `credProtect` extension options
                pub cred_protect:
                    ::core::option::Option<::planus::alloc::boxed::Box<self::CredProtect>>,
                ///  ⚠️  - Browsers do not support this!
                pub uvm: bool,
                ///  ⚠️  - This extension result is always unsigned, and only indicates if the
                ///  browser *requests* a residentKey to be created. It has no bearing on the
                ///  true rk state of the credential.
                pub cred_props: bool,
                ///  CTAP2.1 Minumum pin length
                pub min_pin_length: bool,
                ///  ⚠️  - Browsers support the *creation* of the secret, but not the retrieval of it.
                ///  CTAP2.1 create hmac secret
                pub hmac_create_secret: bool,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for RequestRegistrationExtensions {
                fn default() -> Self {
                    Self {
                        cred_protect: ::core::default::Default::default(),
                        uvm: false,
                        cred_props: false,
                        min_pin_length: false,
                        hmac_create_secret: false,
                    }
                }
            }

            impl RequestRegistrationExtensions {
                /// Creates a [RequestRegistrationExtensionsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> RequestRegistrationExtensionsBuilder<()> {
                    RequestRegistrationExtensionsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_cred_protect: impl ::planus::WriteAsOptional<
                        ::planus::Offset<self::CredProtect>,
                    >,
                    field_uvm: impl ::planus::WriteAsDefault<bool, bool>,
                    field_cred_props: impl ::planus::WriteAsDefault<bool, bool>,
                    field_min_pin_length: impl ::planus::WriteAsDefault<bool, bool>,
                    field_hmac_create_secret: impl ::planus::WriteAsDefault<bool, bool>,
                ) -> ::planus::Offset<Self> {
                    let prepared_cred_protect = field_cred_protect.prepare(builder);
                    let prepared_uvm = field_uvm.prepare(builder, &false);
                    let prepared_cred_props = field_cred_props.prepare(builder, &false);
                    let prepared_min_pin_length = field_min_pin_length.prepare(builder, &false);
                    let prepared_hmac_create_secret =
                        field_hmac_create_secret.prepare(builder, &false);

                    let mut table_writer: ::planus::table_writer::TableWriter<14> =
                        ::core::default::Default::default();
                    if prepared_cred_protect.is_some() {
                        table_writer.write_entry::<::planus::Offset<self::CredProtect>>(0);
                    }
                    if prepared_uvm.is_some() {
                        table_writer.write_entry::<bool>(1);
                    }
                    if prepared_cred_props.is_some() {
                        table_writer.write_entry::<bool>(2);
                    }
                    if prepared_min_pin_length.is_some() {
                        table_writer.write_entry::<bool>(3);
                    }
                    if prepared_hmac_create_secret.is_some() {
                        table_writer.write_entry::<bool>(4);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_cred_protect) =
                                prepared_cred_protect
                            {
                                object_writer.write::<_, _, 4>(&prepared_cred_protect);
                            }
                            if let ::core::option::Option::Some(prepared_uvm) = prepared_uvm {
                                object_writer.write::<_, _, 1>(&prepared_uvm);
                            }
                            if let ::core::option::Option::Some(prepared_cred_props) =
                                prepared_cred_props
                            {
                                object_writer.write::<_, _, 1>(&prepared_cred_props);
                            }
                            if let ::core::option::Option::Some(prepared_min_pin_length) =
                                prepared_min_pin_length
                            {
                                object_writer.write::<_, _, 1>(&prepared_min_pin_length);
                            }
                            if let ::core::option::Option::Some(prepared_hmac_create_secret) =
                                prepared_hmac_create_secret
                            {
                                object_writer.write::<_, _, 1>(&prepared_hmac_create_secret);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensions
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensions
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RequestRegistrationExtensions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RequestRegistrationExtensions> for RequestRegistrationExtensions {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions> {
                    RequestRegistrationExtensions::create(
                        builder,
                        &self.cred_protect,
                        self.uvm,
                        self.cred_props,
                        self.min_pin_length,
                        self.hmac_create_secret,
                    )
                }
            }

            /// Builder for serializing an instance of the [RequestRegistrationExtensions] type.
            ///
            /// Can be created using the [RequestRegistrationExtensions::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct RequestRegistrationExtensionsBuilder<State>(State);

            impl RequestRegistrationExtensionsBuilder<()> {
                /// Setter for the [`cred_protect` field](RequestRegistrationExtensions#structfield.cred_protect).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_protect<T0>(
                    self,
                    value: T0,
                ) -> RequestRegistrationExtensionsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::CredProtect>>,
                {
                    RequestRegistrationExtensionsBuilder((value,))
                }

                /// Sets the [`cred_protect` field](RequestRegistrationExtensions#structfield.cred_protect) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_protect_as_null(self) -> RequestRegistrationExtensionsBuilder<((),)> {
                    self.cred_protect(())
                }
            }

            impl<T0> RequestRegistrationExtensionsBuilder<(T0,)> {
                /// Setter for the [`uvm` field](RequestRegistrationExtensions#structfield.uvm).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn uvm<T1>(self, value: T1) -> RequestRegistrationExtensionsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0,) = self.0;
                    RequestRegistrationExtensionsBuilder((v0, value))
                }

                /// Sets the [`uvm` field](RequestRegistrationExtensions#structfield.uvm) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn uvm_as_default(
                    self,
                ) -> RequestRegistrationExtensionsBuilder<(T0, ::planus::DefaultValue)>
                {
                    self.uvm(::planus::DefaultValue)
                }
            }

            impl<T0, T1> RequestRegistrationExtensionsBuilder<(T0, T1)> {
                /// Setter for the [`cred_props` field](RequestRegistrationExtensions#structfield.cred_props).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_props<T2>(
                    self,
                    value: T2,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1) = self.0;
                    RequestRegistrationExtensionsBuilder((v0, v1, value))
                }

                /// Sets the [`cred_props` field](RequestRegistrationExtensions#structfield.cred_props) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn cred_props_as_default(
                    self,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, ::planus::DefaultValue)>
                {
                    self.cred_props(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2> RequestRegistrationExtensionsBuilder<(T0, T1, T2)> {
                /// Setter for the [`min_pin_length` field](RequestRegistrationExtensions#structfield.min_pin_length).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_pin_length<T3>(
                    self,
                    value: T3,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3)>
                where
                    T3: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1, v2) = self.0;
                    RequestRegistrationExtensionsBuilder((v0, v1, v2, value))
                }

                /// Sets the [`min_pin_length` field](RequestRegistrationExtensions#structfield.min_pin_length) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn min_pin_length_as_default(
                    self,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, T2, ::planus::DefaultValue)>
                {
                    self.min_pin_length(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3> RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3)> {
                /// Setter for the [`hmac_create_secret` field](RequestRegistrationExtensions#structfield.hmac_create_secret).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hmac_create_secret<T4>(
                    self,
                    value: T4,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, T4)>
                where
                    T4: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0, v1, v2, v3) = self.0;
                    RequestRegistrationExtensionsBuilder((v0, v1, v2, v3, value))
                }

                /// Sets the [`hmac_create_secret` field](RequestRegistrationExtensions#structfield.hmac_create_secret) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn hmac_create_secret_as_default(
                    self,
                ) -> RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, ::planus::DefaultValue)>
                {
                    self.hmac_create_secret(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2, T3, T4> RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, T4)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RequestRegistrationExtensions].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions>
                where
                    Self: ::planus::WriteAsOffset<RequestRegistrationExtensions>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::CredProtect>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<bool, bool>,
                    T4: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAs<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, T4)>
            {
                type Prepared = ::planus::Offset<RequestRegistrationExtensions>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::CredProtect>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<bool, bool>,
                    T4: ::planus::WriteAsDefault<bool, bool>,
                >
                ::planus::WriteAsOptional<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, T4)>
            {
                type Prepared = ::planus::Offset<RequestRegistrationExtensions>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RequestRegistrationExtensions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<self::CredProtect>>,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<bool, bool>,
                    T3: ::planus::WriteAsDefault<bool, bool>,
                    T4: ::planus::WriteAsDefault<bool, bool>,
                > ::planus::WriteAsOffset<RequestRegistrationExtensions>
                for RequestRegistrationExtensionsBuilder<(T0, T1, T2, T3, T4)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions> {
                    let (v0, v1, v2, v3, v4) = &self.0;
                    RequestRegistrationExtensions::create(builder, v0, v1, v2, v3, v4)
                }
            }

            /// Reference to a deserialized [RequestRegistrationExtensions].
            #[derive(Copy, Clone)]
            pub struct RequestRegistrationExtensionsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RequestRegistrationExtensionsRef<'a> {
                /// Getter for the [`cred_protect` field](RequestRegistrationExtensions#structfield.cred_protect).
                #[inline]
                pub fn cred_protect(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::CredProtectRef<'a>>>
                {
                    self.0
                        .access(0, "RequestRegistrationExtensions", "cred_protect")
                }

                /// Getter for the [`uvm` field](RequestRegistrationExtensions#structfield.uvm).
                #[inline]
                pub fn uvm(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "RequestRegistrationExtensions", "uvm")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`cred_props` field](RequestRegistrationExtensions#structfield.cred_props).
                #[inline]
                pub fn cred_props(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "RequestRegistrationExtensions", "cred_props")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`min_pin_length` field](RequestRegistrationExtensions#structfield.min_pin_length).
                #[inline]
                pub fn min_pin_length(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(3, "RequestRegistrationExtensions", "min_pin_length")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`hmac_create_secret` field](RequestRegistrationExtensions#structfield.hmac_create_secret).
                #[inline]
                pub fn hmac_create_secret(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(4, "RequestRegistrationExtensions", "hmac_create_secret")?
                            .unwrap_or(false),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for RequestRegistrationExtensionsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("RequestRegistrationExtensionsRef");
                    if let ::core::option::Option::Some(field_cred_protect) =
                        self.cred_protect().transpose()
                    {
                        f.field("cred_protect", &field_cred_protect);
                    }
                    f.field("uvm", &self.uvm());
                    f.field("cred_props", &self.cred_props());
                    f.field("min_pin_length", &self.min_pin_length());
                    f.field("hmac_create_secret", &self.hmac_create_secret());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<RequestRegistrationExtensionsRef<'a>>
                for RequestRegistrationExtensions
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: RequestRegistrationExtensionsRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        cred_protect: if let ::core::option::Option::Some(cred_protect) =
                            value.cred_protect()?
                        {
                            ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                                ::core::convert::TryInto::try_into(cred_protect)?,
                            ))
                        } else {
                            ::core::option::Option::None
                        },
                        uvm: ::core::convert::TryInto::try_into(value.uvm()?)?,
                        cred_props: ::core::convert::TryInto::try_into(value.cred_props()?)?,
                        min_pin_length: ::core::convert::TryInto::try_into(
                            value.min_pin_length()?,
                        )?,
                        hmac_create_secret: ::core::convert::TryInto::try_into(
                            value.hmac_create_secret()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for RequestRegistrationExtensionsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for RequestRegistrationExtensionsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RequestRegistrationExtensionsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensions
            {
                type Value = ::planus::Offset<RequestRegistrationExtensions>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<RequestRegistrationExtensions>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for RequestRegistrationExtensionsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RequestRegistrationExtensionsRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The table `PublicKeyCredentialDescriptor` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `PublicKeyCredentialDescriptor` in the file `auth/webauthn.fbs:114`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct PublicKeyCredentialDescriptor {
                ///  The type of credential to exclude.
                pub type_: ::core::option::Option<::planus::alloc::string::String>,
                ///  The credential ID to exclude.
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                ///  The transports that are allowed for this credential.
                pub transports:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::AuthenticatorTransport>>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for PublicKeyCredentialDescriptor {
                fn default() -> Self {
                    Self {
                        type_: ::core::default::Default::default(),
                        id: ::core::default::Default::default(),
                        transports: ::core::default::Default::default(),
                    }
                }
            }

            impl PublicKeyCredentialDescriptor {
                /// Creates a [PublicKeyCredentialDescriptorBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> PublicKeyCredentialDescriptorBuilder<()> {
                    PublicKeyCredentialDescriptorBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_type_: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                    field_id: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_transports: impl ::planus::WriteAsOptional<
                        ::planus::Offset<[self::AuthenticatorTransport]>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_type_ = field_type_.prepare(builder);
                    let prepared_id = field_id.prepare(builder);
                    let prepared_transports = field_transports.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_type_.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(0);
                    }
                    if prepared_id.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(1);
                    }
                    if prepared_transports.is_some() {
                        table_writer
                            .write_entry::<::planus::Offset<[self::AuthenticatorTransport]>>(2);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                                object_writer.write::<_, _, 4>(&prepared_type_);
                            }
                            if let ::core::option::Option::Some(prepared_id) = prepared_id {
                                object_writer.write::<_, _, 4>(&prepared_id);
                            }
                            if let ::core::option::Option::Some(prepared_transports) =
                                prepared_transports
                            {
                                object_writer.write::<_, _, 4>(&prepared_transports);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptor
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PublicKeyCredentialDescriptor> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptor
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PublicKeyCredentialDescriptor>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<PublicKeyCredentialDescriptor> for PublicKeyCredentialDescriptor {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PublicKeyCredentialDescriptor> {
                    PublicKeyCredentialDescriptor::create(
                        builder,
                        &self.type_,
                        &self.id,
                        &self.transports,
                    )
                }
            }

            /// Builder for serializing an instance of the [PublicKeyCredentialDescriptor] type.
            ///
            /// Can be created using the [PublicKeyCredentialDescriptor::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct PublicKeyCredentialDescriptorBuilder<State>(State);

            impl PublicKeyCredentialDescriptorBuilder<()> {
                /// Setter for the [`type` field](PublicKeyCredentialDescriptor#structfield.type_).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_<T0>(self, value: T0) -> PublicKeyCredentialDescriptorBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    PublicKeyCredentialDescriptorBuilder((value,))
                }

                /// Sets the [`type` field](PublicKeyCredentialDescriptor#structfield.type_) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_as_null(self) -> PublicKeyCredentialDescriptorBuilder<((),)> {
                    self.type_(())
                }
            }

            impl<T0> PublicKeyCredentialDescriptorBuilder<(T0,)> {
                /// Setter for the [`id` field](PublicKeyCredentialDescriptor#structfield.id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id<T1>(self, value: T1) -> PublicKeyCredentialDescriptorBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    let (v0,) = self.0;
                    PublicKeyCredentialDescriptorBuilder((v0, value))
                }

                /// Sets the [`id` field](PublicKeyCredentialDescriptor#structfield.id) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id_as_null(self) -> PublicKeyCredentialDescriptorBuilder<(T0, ())> {
                    self.id(())
                }
            }

            impl<T0, T1> PublicKeyCredentialDescriptorBuilder<(T0, T1)> {
                /// Setter for the [`transports` field](PublicKeyCredentialDescriptor#structfield.transports).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn transports<T2>(
                    self,
                    value: T2,
                ) -> PublicKeyCredentialDescriptorBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                {
                    let (v0, v1) = self.0;
                    PublicKeyCredentialDescriptorBuilder((v0, v1, value))
                }

                /// Sets the [`transports` field](PublicKeyCredentialDescriptor#structfield.transports) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn transports_as_null(
                    self,
                ) -> PublicKeyCredentialDescriptorBuilder<(T0, T1, ())> {
                    self.transports(())
                }
            }

            impl<T0, T1, T2> PublicKeyCredentialDescriptorBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [PublicKeyCredentialDescriptor].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PublicKeyCredentialDescriptor>
                where
                    Self: ::planus::WriteAsOffset<PublicKeyCredentialDescriptor>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                > ::planus::WriteAs<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptorBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<PublicKeyCredentialDescriptor>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PublicKeyCredentialDescriptor> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                >
                ::planus::WriteAsOptional<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptorBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<PublicKeyCredentialDescriptor>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PublicKeyCredentialDescriptor>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<[self::AuthenticatorTransport]>>,
                > ::planus::WriteAsOffset<PublicKeyCredentialDescriptor>
                for PublicKeyCredentialDescriptorBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PublicKeyCredentialDescriptor> {
                    let (v0, v1, v2) = &self.0;
                    PublicKeyCredentialDescriptor::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [PublicKeyCredentialDescriptor].
            #[derive(Copy, Clone)]
            pub struct PublicKeyCredentialDescriptorRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> PublicKeyCredentialDescriptorRef<'a> {
                /// Getter for the [`type` field](PublicKeyCredentialDescriptor#structfield.type_).
                #[inline]
                pub fn type_(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "PublicKeyCredentialDescriptor", "type_")
                }

                /// Getter for the [`id` field](PublicKeyCredentialDescriptor#structfield.id).
                #[inline]
                pub fn id(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(1, "PublicKeyCredentialDescriptor", "id")
                }

                /// Getter for the [`transports` field](PublicKeyCredentialDescriptor#structfield.transports).
                #[inline]
                pub fn transports(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<
                        ::planus::Vector<
                            'a,
                            ::core::result::Result<
                                self::AuthenticatorTransport,
                                ::planus::errors::UnknownEnumTag,
                            >,
                        >,
                    >,
                > {
                    self.0
                        .access(2, "PublicKeyCredentialDescriptor", "transports")
                }
            }

            impl<'a> ::core::fmt::Debug for PublicKeyCredentialDescriptorRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("PublicKeyCredentialDescriptorRef");
                    if let ::core::option::Option::Some(field_type_) = self.type_().transpose() {
                        f.field("type_", &field_type_);
                    }
                    if let ::core::option::Option::Some(field_id) = self.id().transpose() {
                        f.field("id", &field_id);
                    }
                    if let ::core::option::Option::Some(field_transports) =
                        self.transports().transpose()
                    {
                        f.field("transports", &field_transports);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<PublicKeyCredentialDescriptorRef<'a>>
                for PublicKeyCredentialDescriptor
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: PublicKeyCredentialDescriptorRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        type_: if let ::core::option::Option::Some(type_) = value.type_()? {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(type_)?)
                        } else {
                            ::core::option::Option::None
                        },
                        id: value.id()?.map(|v| v.to_vec()),
                        transports: if let ::core::option::Option::Some(transports) =
                            value.transports()?
                        {
                            ::core::option::Option::Some(transports.to_vec_result()?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for PublicKeyCredentialDescriptorRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for PublicKeyCredentialDescriptorRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[PublicKeyCredentialDescriptorRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptor
            {
                type Value = ::planus::Offset<PublicKeyCredentialDescriptor>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<PublicKeyCredentialDescriptor>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for PublicKeyCredentialDescriptorRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[PublicKeyCredentialDescriptorRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The enum `AuthenticatorAttachment` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Enum `AuthenticatorAttachment` in the file `auth/webauthn.fbs:123`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i8)]
            pub enum AuthenticatorAttachment {
                ///  Request a device that is part of the machine aka inseperable.
                ///  <https://www.w3.org/TR/webauthn/#attachment>
                Platform = 0,

                ///  Request a device that can be seperated from the machine aka an external token.
                ///  <https://www.w3.org/TR/webauthn/#attachment>
                CrossPlatform = 1,
            }

            impl AuthenticatorAttachment {
                /// Array containing all valid variants of AuthenticatorAttachment
                pub const ENUM_VALUES: [Self; 2] = [Self::Platform, Self::CrossPlatform];
            }

            impl ::core::convert::TryFrom<i8> for AuthenticatorAttachment {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(AuthenticatorAttachment::Platform),
                        1 => ::core::result::Result::Ok(AuthenticatorAttachment::CrossPlatform),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<AuthenticatorAttachment> for i8 {
                #[inline]
                fn from(value: AuthenticatorAttachment) -> Self {
                    value as i8
                }
            }

            impl ::planus::Primitive for AuthenticatorAttachment {
                const ALIGNMENT: usize = 1;
                const SIZE: usize = 1;
            }

            impl ::planus::WriteAsPrimitive<AuthenticatorAttachment> for AuthenticatorAttachment {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i8).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<AuthenticatorAttachment> for AuthenticatorAttachment {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> AuthenticatorAttachment {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<AuthenticatorAttachment, AuthenticatorAttachment>
                for AuthenticatorAttachment
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &AuthenticatorAttachment,
                ) -> ::core::option::Option<AuthenticatorAttachment> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<AuthenticatorAttachment> for AuthenticatorAttachment {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<AuthenticatorAttachment> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for AuthenticatorAttachment {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for AuthenticatorAttachment {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 1;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value = *buffer.buffer.get_unchecked(offset) as i8;
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "AuthenticatorAttachment",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<AuthenticatorAttachment> for AuthenticatorAttachment {
                const STRIDE: usize = 1;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - i as u32,
                        );
                    }
                }
            }

            /// The enum `UserVerificationPolicy` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Enum `UserVerificationPolicy` in the file `auth/webauthn.fbs:132`
            #[derive(
                Copy,
                Clone,
                Debug,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            #[repr(i8)]
            pub enum UserVerificationPolicy {
                ///  <https://www.w3.org/TR/webauthn/#dom-userverificationrequirement-preferred>
                Preferred = 0,

                ///  <https://www.w3.org/TR/webauthn/#dom-userverificationrequirement-required>
                Required = 1,

                ///  <https://www.w3.org/TR/webauthn/#dom-userverificationrequirement-discouraged>
                Discouraged = 2,
            }

            impl UserVerificationPolicy {
                /// Array containing all valid variants of UserVerificationPolicy
                pub const ENUM_VALUES: [Self; 3] =
                    [Self::Preferred, Self::Required, Self::Discouraged];
            }

            impl ::core::convert::TryFrom<i8> for UserVerificationPolicy {
                type Error = ::planus::errors::UnknownEnumTagKind;
                #[inline]
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
                    match value {
                        0 => ::core::result::Result::Ok(UserVerificationPolicy::Preferred),
                        1 => ::core::result::Result::Ok(UserVerificationPolicy::Required),
                        2 => ::core::result::Result::Ok(UserVerificationPolicy::Discouraged),

                        _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind {
                            tag: value as i128,
                        }),
                    }
                }
            }

            impl ::core::convert::From<UserVerificationPolicy> for i8 {
                #[inline]
                fn from(value: UserVerificationPolicy) -> Self {
                    value as i8
                }
            }

            impl ::planus::Primitive for UserVerificationPolicy {
                const ALIGNMENT: usize = 1;
                const SIZE: usize = 1;
            }

            impl ::planus::WriteAsPrimitive<UserVerificationPolicy> for UserVerificationPolicy {
                #[inline]
                fn write<const N: usize>(
                    &self,
                    cursor: ::planus::Cursor<'_, N>,
                    buffer_position: u32,
                ) {
                    (*self as i8).write(cursor, buffer_position);
                }
            }

            impl ::planus::WriteAs<UserVerificationPolicy> for UserVerificationPolicy {
                type Prepared = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> UserVerificationPolicy {
                    *self
                }
            }

            impl ::planus::WriteAsDefault<UserVerificationPolicy, UserVerificationPolicy>
                for UserVerificationPolicy
            {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                    default: &UserVerificationPolicy,
                ) -> ::core::option::Option<UserVerificationPolicy> {
                    if self == default {
                        ::core::option::Option::None
                    } else {
                        ::core::option::Option::Some(*self)
                    }
                }
            }

            impl ::planus::WriteAsOptional<UserVerificationPolicy> for UserVerificationPolicy {
                type Prepared = Self;

                #[inline]
                fn prepare(
                    &self,
                    _builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<UserVerificationPolicy> {
                    ::core::option::Option::Some(*self)
                }
            }

            impl<'buf> ::planus::TableRead<'buf> for UserVerificationPolicy {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    let n: i8 = ::planus::TableRead::from_buffer(buffer, offset)?;
                    ::core::result::Result::Ok(::core::convert::TryInto::try_into(n)?)
                }
            }

            impl<'buf> ::planus::VectorReadInner<'buf> for UserVerificationPolicy {
                type Error = ::planus::errors::UnknownEnumTag;
                const STRIDE: usize = 1;
                #[inline]
                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'buf>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTag>
                {
                    let value = *buffer.buffer.get_unchecked(offset) as i8;
                    let value: ::core::result::Result<Self, _> =
                        ::core::convert::TryInto::try_into(value);
                    value.map_err(|error_kind| {
                        error_kind.with_error_location(
                            "UserVerificationPolicy",
                            "VectorRead::from_buffer",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<UserVerificationPolicy> for UserVerificationPolicy {
                const STRIDE: usize = 1;

                type Value = Self;

                #[inline]
                fn prepare(&self, _builder: &mut ::planus::Builder) -> Self {
                    *self
                }

                #[inline]
                unsafe fn write_values(
                    values: &[Self],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 1];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - i as u32,
                        );
                    }
                }
            }

            /// The table `AuthenticatorSelectionCriteria` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `AuthenticatorSelectionCriteria` in the file `auth/webauthn.fbs:141`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct AuthenticatorSelectionCriteria {
                ///  The authenticator attachment mode.
                pub authenticator_attachment: self::AuthenticatorAttachment,
                ///  Whether or not the authenticator must be resident.
                pub require_resident_key: bool,
                ///  The list of authenticator transports that are allowed.
                pub user_verification: self::UserVerificationPolicy,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for AuthenticatorSelectionCriteria {
                fn default() -> Self {
                    Self {
                        authenticator_attachment: self::AuthenticatorAttachment::Platform,
                        require_resident_key: false,
                        user_verification: self::UserVerificationPolicy::Preferred,
                    }
                }
            }

            impl AuthenticatorSelectionCriteria {
                /// Creates a [AuthenticatorSelectionCriteriaBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> AuthenticatorSelectionCriteriaBuilder<()> {
                    AuthenticatorSelectionCriteriaBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_authenticator_attachment: impl ::planus::WriteAsDefault<
                        self::AuthenticatorAttachment,
                        self::AuthenticatorAttachment,
                    >,
                    field_require_resident_key: impl ::planus::WriteAsDefault<bool, bool>,
                    field_user_verification: impl ::planus::WriteAsDefault<
                        self::UserVerificationPolicy,
                        self::UserVerificationPolicy,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_authenticator_attachment = field_authenticator_attachment
                        .prepare(builder, &self::AuthenticatorAttachment::Platform);
                    let prepared_require_resident_key =
                        field_require_resident_key.prepare(builder, &false);
                    let prepared_user_verification = field_user_verification
                        .prepare(builder, &self::UserVerificationPolicy::Preferred);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_authenticator_attachment.is_some() {
                        table_writer.write_entry::<self::AuthenticatorAttachment>(0);
                    }
                    if prepared_require_resident_key.is_some() {
                        table_writer.write_entry::<bool>(1);
                    }
                    if prepared_user_verification.is_some() {
                        table_writer.write_entry::<self::UserVerificationPolicy>(2);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_authenticator_attachment) =
                                prepared_authenticator_attachment
                            {
                                object_writer.write::<_, _, 1>(&prepared_authenticator_attachment);
                            }
                            if let ::core::option::Option::Some(prepared_require_resident_key) =
                                prepared_require_resident_key
                            {
                                object_writer.write::<_, _, 1>(&prepared_require_resident_key);
                            }
                            if let ::core::option::Option::Some(prepared_user_verification) =
                                prepared_user_verification
                            {
                                object_writer.write::<_, _, 1>(&prepared_user_verification);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteria
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteria
            {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<AuthenticatorSelectionCriteria>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<AuthenticatorSelectionCriteria> for AuthenticatorSelectionCriteria {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria> {
                    AuthenticatorSelectionCriteria::create(
                        builder,
                        self.authenticator_attachment,
                        self.require_resident_key,
                        self.user_verification,
                    )
                }
            }

            /// Builder for serializing an instance of the [AuthenticatorSelectionCriteria] type.
            ///
            /// Can be created using the [AuthenticatorSelectionCriteria::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct AuthenticatorSelectionCriteriaBuilder<State>(State);

            impl AuthenticatorSelectionCriteriaBuilder<()> {
                /// Setter for the [`authenticator_attachment` field](AuthenticatorSelectionCriteria#structfield.authenticator_attachment).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn authenticator_attachment<T0>(
                    self,
                    value: T0,
                ) -> AuthenticatorSelectionCriteriaBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsDefault<
                        self::AuthenticatorAttachment,
                        self::AuthenticatorAttachment,
                    >,
                {
                    AuthenticatorSelectionCriteriaBuilder((value,))
                }

                /// Sets the [`authenticator_attachment` field](AuthenticatorSelectionCriteria#structfield.authenticator_attachment) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn authenticator_attachment_as_default(
                    self,
                ) -> AuthenticatorSelectionCriteriaBuilder<(::planus::DefaultValue,)>
                {
                    self.authenticator_attachment(::planus::DefaultValue)
                }
            }

            impl<T0> AuthenticatorSelectionCriteriaBuilder<(T0,)> {
                /// Setter for the [`require_resident_key` field](AuthenticatorSelectionCriteria#structfield.require_resident_key).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn require_resident_key<T1>(
                    self,
                    value: T1,
                ) -> AuthenticatorSelectionCriteriaBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<bool, bool>,
                {
                    let (v0,) = self.0;
                    AuthenticatorSelectionCriteriaBuilder((v0, value))
                }

                /// Sets the [`require_resident_key` field](AuthenticatorSelectionCriteria#structfield.require_resident_key) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn require_resident_key_as_default(
                    self,
                ) -> AuthenticatorSelectionCriteriaBuilder<(T0, ::planus::DefaultValue)>
                {
                    self.require_resident_key(::planus::DefaultValue)
                }
            }

            impl<T0, T1> AuthenticatorSelectionCriteriaBuilder<(T0, T1)> {
                /// Setter for the [`user_verification` field](AuthenticatorSelectionCriteria#structfield.user_verification).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn user_verification<T2>(
                    self,
                    value: T2,
                ) -> AuthenticatorSelectionCriteriaBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsDefault<
                        self::UserVerificationPolicy,
                        self::UserVerificationPolicy,
                    >,
                {
                    let (v0, v1) = self.0;
                    AuthenticatorSelectionCriteriaBuilder((v0, v1, value))
                }

                /// Sets the [`user_verification` field](AuthenticatorSelectionCriteria#structfield.user_verification) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn user_verification_as_default(
                    self,
                ) -> AuthenticatorSelectionCriteriaBuilder<(T0, T1, ::planus::DefaultValue)>
                {
                    self.user_verification(::planus::DefaultValue)
                }
            }

            impl<T0, T1, T2> AuthenticatorSelectionCriteriaBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [AuthenticatorSelectionCriteria].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria>
                where
                    Self: ::planus::WriteAsOffset<AuthenticatorSelectionCriteria>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::AuthenticatorAttachment,
                        self::AuthenticatorAttachment,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<
                        self::UserVerificationPolicy,
                        self::UserVerificationPolicy,
                    >,
                > ::planus::WriteAs<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteriaBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<AuthenticatorSelectionCriteria>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::AuthenticatorAttachment,
                        self::AuthenticatorAttachment,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<
                        self::UserVerificationPolicy,
                        self::UserVerificationPolicy,
                    >,
                >
                ::planus::WriteAsOptional<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteriaBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<AuthenticatorSelectionCriteria>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<AuthenticatorSelectionCriteria>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsDefault<
                        self::AuthenticatorAttachment,
                        self::AuthenticatorAttachment,
                    >,
                    T1: ::planus::WriteAsDefault<bool, bool>,
                    T2: ::planus::WriteAsDefault<
                        self::UserVerificationPolicy,
                        self::UserVerificationPolicy,
                    >,
                > ::planus::WriteAsOffset<AuthenticatorSelectionCriteria>
                for AuthenticatorSelectionCriteriaBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria> {
                    let (v0, v1, v2) = &self.0;
                    AuthenticatorSelectionCriteria::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [AuthenticatorSelectionCriteria].
            #[derive(Copy, Clone)]
            pub struct AuthenticatorSelectionCriteriaRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> AuthenticatorSelectionCriteriaRef<'a> {
                /// Getter for the [`authenticator_attachment` field](AuthenticatorSelectionCriteria#structfield.authenticator_attachment).
                #[inline]
                pub fn authenticator_attachment(
                    &self,
                ) -> ::planus::Result<self::AuthenticatorAttachment> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(
                                0,
                                "AuthenticatorSelectionCriteria",
                                "authenticator_attachment",
                            )?
                            .unwrap_or(self::AuthenticatorAttachment::Platform),
                    )
                }

                /// Getter for the [`require_resident_key` field](AuthenticatorSelectionCriteria#structfield.require_resident_key).
                #[inline]
                pub fn require_resident_key(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "AuthenticatorSelectionCriteria", "require_resident_key")?
                            .unwrap_or(false),
                    )
                }

                /// Getter for the [`user_verification` field](AuthenticatorSelectionCriteria#structfield.user_verification).
                #[inline]
                pub fn user_verification(&self) -> ::planus::Result<self::UserVerificationPolicy> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "AuthenticatorSelectionCriteria", "user_verification")?
                            .unwrap_or(self::UserVerificationPolicy::Preferred),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for AuthenticatorSelectionCriteriaRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("AuthenticatorSelectionCriteriaRef");
                    f.field("authenticator_attachment", &self.authenticator_attachment());
                    f.field("require_resident_key", &self.require_resident_key());
                    f.field("user_verification", &self.user_verification());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<AuthenticatorSelectionCriteriaRef<'a>>
                for AuthenticatorSelectionCriteria
            {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(
                    value: AuthenticatorSelectionCriteriaRef<'a>,
                ) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        authenticator_attachment: ::core::convert::TryInto::try_into(
                            value.authenticator_attachment()?,
                        )?,
                        require_resident_key: ::core::convert::TryInto::try_into(
                            value.require_resident_key()?,
                        )?,
                        user_verification: ::core::convert::TryInto::try_into(
                            value.user_verification()?,
                        )?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for AuthenticatorSelectionCriteriaRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for AuthenticatorSelectionCriteriaRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[AuthenticatorSelectionCriteriaRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteria
            {
                type Value = ::planus::Offset<AuthenticatorSelectionCriteria>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<AuthenticatorSelectionCriteria>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for AuthenticatorSelectionCriteriaRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[AuthenticatorSelectionCriteriaRef]",
                            "read_as_root",
                            0,
                        )
                    })
                }
            }

            /// The table `PubKeyCredParams` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `PubKeyCredParams` in the file `auth/webauthn.fbs:150`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct PubKeyCredParams {
                /// The field `type` in the table `PubKeyCredParams`
                pub type_: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `alg` in the table `PubKeyCredParams`
                pub alg: i32,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for PubKeyCredParams {
                fn default() -> Self {
                    Self {
                        type_: ::core::default::Default::default(),
                        alg: 0,
                    }
                }
            }

            impl PubKeyCredParams {
                /// Creates a [PubKeyCredParamsBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> PubKeyCredParamsBuilder<()> {
                    PubKeyCredParamsBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_type_: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                    field_alg: impl ::planus::WriteAsDefault<i32, i32>,
                ) -> ::planus::Offset<Self> {
                    let prepared_type_ = field_type_.prepare(builder);
                    let prepared_alg = field_alg.prepare(builder, &0);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_type_.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(0);
                    }
                    if prepared_alg.is_some() {
                        table_writer.write_entry::<i32>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                                object_writer.write::<_, _, 4>(&prepared_type_);
                            }
                            if let ::core::option::Option::Some(prepared_alg) = prepared_alg {
                                object_writer.write::<_, _, 4>(&prepared_alg);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<PubKeyCredParams>> for PubKeyCredParams {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<PubKeyCredParams>> for PubKeyCredParams {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PubKeyCredParams>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<PubKeyCredParams> for PubKeyCredParams {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    PubKeyCredParams::create(builder, &self.type_, self.alg)
                }
            }

            /// Builder for serializing an instance of the [PubKeyCredParams] type.
            ///
            /// Can be created using the [PubKeyCredParams::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct PubKeyCredParamsBuilder<State>(State);

            impl PubKeyCredParamsBuilder<()> {
                /// Setter for the [`type` field](PubKeyCredParams#structfield.type_).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_<T0>(self, value: T0) -> PubKeyCredParamsBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    PubKeyCredParamsBuilder((value,))
                }

                /// Sets the [`type` field](PubKeyCredParams#structfield.type_) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn type_as_null(self) -> PubKeyCredParamsBuilder<((),)> {
                    self.type_(())
                }
            }

            impl<T0> PubKeyCredParamsBuilder<(T0,)> {
                /// Setter for the [`alg` field](PubKeyCredParams#structfield.alg).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn alg<T1>(self, value: T1) -> PubKeyCredParamsBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsDefault<i32, i32>,
                {
                    let (v0,) = self.0;
                    PubKeyCredParamsBuilder((v0, value))
                }

                /// Sets the [`alg` field](PubKeyCredParams#structfield.alg) to the default value.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn alg_as_default(
                    self,
                ) -> PubKeyCredParamsBuilder<(T0, ::planus::DefaultValue)> {
                    self.alg(::planus::DefaultValue)
                }
            }

            impl<T0, T1> PubKeyCredParamsBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [PubKeyCredParams].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams>
                where
                    Self: ::planus::WriteAsOffset<PubKeyCredParams>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAs<::planus::Offset<PubKeyCredParams>>
                for PubKeyCredParamsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<PubKeyCredParams>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOptional<::planus::Offset<PubKeyCredParams>>
                for PubKeyCredParamsBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<PubKeyCredParams>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PubKeyCredParams>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsDefault<i32, i32>,
                > ::planus::WriteAsOffset<PubKeyCredParams> for PubKeyCredParamsBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    let (v0, v1) = &self.0;
                    PubKeyCredParams::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [PubKeyCredParams].
            #[derive(Copy, Clone)]
            pub struct PubKeyCredParamsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> PubKeyCredParamsRef<'a> {
                /// Getter for the [`type` field](PubKeyCredParams#structfield.type_).
                #[inline]
                pub fn type_(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "PubKeyCredParams", "type_")
                }

                /// Getter for the [`alg` field](PubKeyCredParams#structfield.alg).
                #[inline]
                pub fn alg(&self) -> ::planus::Result<i32> {
                    ::core::result::Result::Ok(
                        self.0.access(1, "PubKeyCredParams", "alg")?.unwrap_or(0),
                    )
                }
            }

            impl<'a> ::core::fmt::Debug for PubKeyCredParamsRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("PubKeyCredParamsRef");
                    if let ::core::option::Option::Some(field_type_) = self.type_().transpose() {
                        f.field("type_", &field_type_);
                    }
                    f.field("alg", &self.alg());
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<PubKeyCredParamsRef<'a>> for PubKeyCredParams {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: PubKeyCredParamsRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        type_: if let ::core::option::Option::Some(type_) = value.type_()? {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(type_)?)
                        } else {
                            ::core::option::Option::None
                        },
                        alg: ::core::convert::TryInto::try_into(value.alg()?)?,
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for PubKeyCredParamsRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for PubKeyCredParamsRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[PubKeyCredParamsRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<PubKeyCredParams>> for PubKeyCredParams {
                type Value = ::planus::Offset<PubKeyCredParams>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<PubKeyCredParams>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for PubKeyCredParamsRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[PubKeyCredParamsRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `RelyingParty` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `RelyingParty` in the file `auth/webauthn.fbs:155`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct RelyingParty {
                /// The field `id` in the table `RelyingParty`
                pub id: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `name` in the table `RelyingParty`
                pub name: ::core::option::Option<::planus::alloc::string::String>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for RelyingParty {
                fn default() -> Self {
                    Self {
                        id: ::core::default::Default::default(),
                        name: ::core::default::Default::default(),
                    }
                }
            }

            impl RelyingParty {
                /// Creates a [RelyingPartyBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> RelyingPartyBuilder<()> {
                    RelyingPartyBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_id: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_id = field_id.prepare(builder);
                    let prepared_name = field_name.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<8> =
                        ::core::default::Default::default();
                    if prepared_id.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(0);
                    }
                    if prepared_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(1);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_id) = prepared_id {
                                object_writer.write::<_, _, 4>(&prepared_id);
                            }
                            if let ::core::option::Option::Some(prepared_name) = prepared_name {
                                object_writer.write::<_, _, 4>(&prepared_name);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RelyingParty>> for RelyingParty {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<RelyingParty>> for RelyingParty {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RelyingParty>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RelyingParty> for RelyingParty {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    RelyingParty::create(builder, &self.id, &self.name)
                }
            }

            /// Builder for serializing an instance of the [RelyingParty] type.
            ///
            /// Can be created using the [RelyingParty::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct RelyingPartyBuilder<State>(State);

            impl RelyingPartyBuilder<()> {
                /// Setter for the [`id` field](RelyingParty#structfield.id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id<T0>(self, value: T0) -> RelyingPartyBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    RelyingPartyBuilder((value,))
                }

                /// Sets the [`id` field](RelyingParty#structfield.id) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id_as_null(self) -> RelyingPartyBuilder<((),)> {
                    self.id(())
                }
            }

            impl<T0> RelyingPartyBuilder<(T0,)> {
                /// Setter for the [`name` field](RelyingParty#structfield.name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name<T1>(self, value: T1) -> RelyingPartyBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0,) = self.0;
                    RelyingPartyBuilder((v0, value))
                }

                /// Sets the [`name` field](RelyingParty#structfield.name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name_as_null(self) -> RelyingPartyBuilder<(T0, ())> {
                    self.name(())
                }
            }

            impl<T0, T1> RelyingPartyBuilder<(T0, T1)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RelyingParty].
                #[inline]
                pub fn finish(
                    self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty>
                where
                    Self: ::planus::WriteAsOffset<RelyingParty>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAs<::planus::Offset<RelyingParty>>
                for RelyingPartyBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<RelyingParty>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOptional<::planus::Offset<RelyingParty>>
                for RelyingPartyBuilder<(T0, T1)>
            {
                type Prepared = ::planus::Offset<RelyingParty>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RelyingParty>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOffset<RelyingParty> for RelyingPartyBuilder<(T0, T1)>
            {
                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    let (v0, v1) = &self.0;
                    RelyingParty::create(builder, v0, v1)
                }
            }

            /// Reference to a deserialized [RelyingParty].
            #[derive(Copy, Clone)]
            pub struct RelyingPartyRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RelyingPartyRef<'a> {
                /// Getter for the [`id` field](RelyingParty#structfield.id).
                #[inline]
                pub fn id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "RelyingParty", "id")
                }

                /// Getter for the [`name` field](RelyingParty#structfield.name).
                #[inline]
                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(1, "RelyingParty", "name")
                }
            }

            impl<'a> ::core::fmt::Debug for RelyingPartyRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("RelyingPartyRef");
                    if let ::core::option::Option::Some(field_id) = self.id().transpose() {
                        f.field("id", &field_id);
                    }
                    if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                        f.field("name", &field_name);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<RelyingPartyRef<'a>> for RelyingParty {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: RelyingPartyRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        id: if let ::core::option::Option::Some(id) = value.id()? {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(id)?)
                        } else {
                            ::core::option::Option::None
                        },
                        name: if let ::core::option::Option::Some(name) = value.name()? {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(name)?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for RelyingPartyRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for RelyingPartyRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location(
                            "[RelyingPartyRef]",
                            "get",
                            buffer.offset_from_start,
                        )
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<RelyingParty>> for RelyingParty {
                type Value = ::planus::Offset<RelyingParty>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<RelyingParty>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for RelyingPartyRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[RelyingPartyRef]", "read_as_root", 0)
                    })
                }
            }

            /// The table `User` in the namespace `Auth.WebAuthn`
            ///
            /// Generated from these locations:
            /// * Table `User` in the file `auth/webauthn.fbs:160`
            #[derive(
                Clone,
                Debug,
                PartialEq,
                PartialOrd,
                Eq,
                Ord,
                Hash,
                ::serde::Serialize,
                ::serde::Deserialize,
            )]
            pub struct User {
                /// The field `id` in the table `User`
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                /// The field `name` in the table `User`
                pub name: ::core::option::Option<::planus::alloc::string::String>,
                /// The field `display_name` in the table `User`
                pub display_name: ::core::option::Option<::planus::alloc::string::String>,
            }

            #[allow(clippy::derivable_impls)]
            impl ::core::default::Default for User {
                fn default() -> Self {
                    Self {
                        id: ::core::default::Default::default(),
                        name: ::core::default::Default::default(),
                        display_name: ::core::default::Default::default(),
                    }
                }
            }

            impl User {
                /// Creates a [UserBuilder] for serializing an instance of this table.
                #[inline]
                pub fn builder() -> UserBuilder<()> {
                    UserBuilder(())
                }

                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_id: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_display_name: impl ::planus::WriteAsOptional<
                        ::planus::Offset<::core::primitive::str>,
                    >,
                ) -> ::planus::Offset<Self> {
                    let prepared_id = field_id.prepare(builder);
                    let prepared_name = field_name.prepare(builder);
                    let prepared_display_name = field_display_name.prepare(builder);

                    let mut table_writer: ::planus::table_writer::TableWriter<10> =
                        ::core::default::Default::default();
                    if prepared_id.is_some() {
                        table_writer.write_entry::<::planus::Offset<[u8]>>(0);
                    }
                    if prepared_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(1);
                    }
                    if prepared_display_name.is_some() {
                        table_writer.write_entry::<::planus::Offset<str>>(2);
                    }

                    unsafe {
                        table_writer.finish(builder, |object_writer| {
                            if let ::core::option::Option::Some(prepared_id) = prepared_id {
                                object_writer.write::<_, _, 4>(&prepared_id);
                            }
                            if let ::core::option::Option::Some(prepared_name) = prepared_name {
                                object_writer.write::<_, _, 4>(&prepared_name);
                            }
                            if let ::core::option::Option::Some(prepared_display_name) =
                                prepared_display_name
                            {
                                object_writer.write::<_, _, 4>(&prepared_display_name);
                            }
                        });
                    }
                    builder.current_offset()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<User>> for User {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<User>> for User {
                type Prepared = ::planus::Offset<Self>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<User>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<User> for User {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    User::create(builder, &self.id, &self.name, &self.display_name)
                }
            }

            /// Builder for serializing an instance of the [User] type.
            ///
            /// Can be created using the [User::builder] method.
            #[derive(Debug)]
            #[must_use]
            pub struct UserBuilder<State>(State);

            impl UserBuilder<()> {
                /// Setter for the [`id` field](User#structfield.id).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id<T0>(self, value: T0) -> UserBuilder<(T0,)>
                where
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                {
                    UserBuilder((value,))
                }

                /// Sets the [`id` field](User#structfield.id) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn id_as_null(self) -> UserBuilder<((),)> {
                    self.id(())
                }
            }

            impl<T0> UserBuilder<(T0,)> {
                /// Setter for the [`name` field](User#structfield.name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name<T1>(self, value: T1) -> UserBuilder<(T0, T1)>
                where
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0,) = self.0;
                    UserBuilder((v0, value))
                }

                /// Sets the [`name` field](User#structfield.name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn name_as_null(self) -> UserBuilder<(T0, ())> {
                    self.name(())
                }
            }

            impl<T0, T1> UserBuilder<(T0, T1)> {
                /// Setter for the [`display_name` field](User#structfield.display_name).
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn display_name<T2>(self, value: T2) -> UserBuilder<(T0, T1, T2)>
                where
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                {
                    let (v0, v1) = self.0;
                    UserBuilder((v0, v1, value))
                }

                /// Sets the [`display_name` field](User#structfield.display_name) to null.
                #[inline]
                #[allow(clippy::type_complexity)]
                pub fn display_name_as_null(self) -> UserBuilder<(T0, T1, ())> {
                    self.display_name(())
                }
            }

            impl<T0, T1, T2> UserBuilder<(T0, T1, T2)> {
                /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [User].
                #[inline]
                pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<User>
                where
                    Self: ::planus::WriteAsOffset<User>,
                {
                    ::planus::WriteAsOffset::prepare(&self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAs<::planus::Offset<User>> for UserBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<User>;

                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOptional<::planus::Offset<User>> for UserBuilder<(T0, T1, T2)>
            {
                type Prepared = ::planus::Offset<User>;

                #[inline]
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<User>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl<
                    T0: ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
                    T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    T2: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                > ::planus::WriteAsOffset<User> for UserBuilder<(T0, T1, T2)>
            {
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    let (v0, v1, v2) = &self.0;
                    User::create(builder, v0, v1, v2)
                }
            }

            /// Reference to a deserialized [User].
            #[derive(Copy, Clone)]
            pub struct UserRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> UserRef<'a> {
                /// Getter for the [`id` field](User#structfield.id).
                #[inline]
                pub fn id(&self) -> ::planus::Result<::core::option::Option<&'a [u8]>> {
                    self.0.access(0, "User", "id")
                }

                /// Getter for the [`name` field](User#structfield.name).
                #[inline]
                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(1, "User", "name")
                }

                /// Getter for the [`display_name` field](User#structfield.display_name).
                #[inline]
                pub fn display_name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(2, "User", "display_name")
                }
            }

            impl<'a> ::core::fmt::Debug for UserRef<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let mut f = f.debug_struct("UserRef");
                    if let ::core::option::Option::Some(field_id) = self.id().transpose() {
                        f.field("id", &field_id);
                    }
                    if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                        f.field("name", &field_name);
                    }
                    if let ::core::option::Option::Some(field_display_name) =
                        self.display_name().transpose()
                    {
                        f.field("display_name", &field_display_name);
                    }
                    f.finish()
                }
            }

            impl<'a> ::core::convert::TryFrom<UserRef<'a>> for User {
                type Error = ::planus::Error;

                #[allow(unreachable_code)]
                fn try_from(value: UserRef<'a>) -> ::planus::Result<Self> {
                    ::core::result::Result::Ok(Self {
                        id: value.id()?.map(|v| v.to_vec()),
                        name: if let ::core::option::Option::Some(name) = value.name()? {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(name)?)
                        } else {
                            ::core::option::Option::None
                        },
                        display_name: if let ::core::option::Option::Some(display_name) =
                            value.display_name()?
                        {
                            ::core::option::Option::Some(::core::convert::TryInto::try_into(
                                display_name,
                            )?)
                        } else {
                            ::core::option::Option::None
                        },
                    })
                }
            }

            impl<'a> ::planus::TableRead<'a> for UserRef<'a> {
                #[inline]
                fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                    ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                        buffer, offset,
                    )?))
                }
            }

            impl<'a> ::planus::VectorReadInner<'a> for UserRef<'a> {
                type Error = ::planus::Error;
                const STRIDE: usize = 4;

                unsafe fn from_buffer(
                    buffer: ::planus::SliceWithStartOffset<'a>,
                    offset: usize,
                ) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                        error_kind.with_error_location("[UserRef]", "get", buffer.offset_from_start)
                    })
                }
            }

            impl ::planus::VectorWrite<::planus::Offset<User>> for User {
                type Value = ::planus::Offset<User>;
                const STRIDE: usize = 4;
                #[inline]
                fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                    ::planus::WriteAs::prepare(self, builder)
                }

                #[inline]
                unsafe fn write_values(
                    values: &[::planus::Offset<User>],
                    bytes: *mut ::core::mem::MaybeUninit<u8>,
                    buffer_position: u32,
                ) {
                    let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                    for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                        ::planus::WriteAsPrimitive::write(
                            v,
                            ::planus::Cursor::new(&mut *bytes.add(i)),
                            buffer_position - (Self::STRIDE * i) as u32,
                        );
                    }
                }
            }

            impl<'a> ::planus::ReadAsRoot<'a> for UserRef<'a> {
                fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                    ::planus::TableRead::from_buffer(
                        ::planus::SliceWithStartOffset {
                            buffer: slice,
                            offset_from_start: 0,
                        },
                        0,
                    )
                    .map_err(|error_kind| {
                        error_kind.with_error_location("[UserRef]", "read_as_root", 0)
                    })
                }
            }
        }
        /// The table `ErrorRes` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `ErrorRes` in the file `auth/auth.fbs:5`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct ErrorRes {
            /// The field `error` in the table `ErrorRes`
            pub error: ::core::option::Option<::planus::alloc::string::String>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for ErrorRes {
            fn default() -> Self {
                Self {
                    error: ::core::default::Default::default(),
                }
            }
        }

        impl ErrorRes {
            /// Creates a [ErrorResBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> ErrorResBuilder<()> {
                ErrorResBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_error: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            ) -> ::planus::Offset<Self> {
                let prepared_error = field_error.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<6> =
                    ::core::default::Default::default();
                if prepared_error.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_error) = prepared_error {
                            object_writer.write::<_, _, 4>(&prepared_error);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<ErrorRes>> for ErrorRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<ErrorRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<ErrorRes>> for ErrorRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<ErrorRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<ErrorRes> for ErrorRes {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<ErrorRes> {
                ErrorRes::create(builder, &self.error)
            }
        }

        /// Builder for serializing an instance of the [ErrorRes] type.
        ///
        /// Can be created using the [ErrorRes::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct ErrorResBuilder<State>(State);

        impl ErrorResBuilder<()> {
            /// Setter for the [`error` field](ErrorRes#structfield.error).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn error<T0>(self, value: T0) -> ErrorResBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                ErrorResBuilder((value,))
            }

            /// Sets the [`error` field](ErrorRes#structfield.error) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn error_as_null(self) -> ErrorResBuilder<((),)> {
                self.error(())
            }
        }

        impl<T0> ErrorResBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [ErrorRes].
            #[inline]
            pub fn finish(self, builder: &mut ::planus::Builder) -> ::planus::Offset<ErrorRes>
            where
                Self: ::planus::WriteAsOffset<ErrorRes>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAs<::planus::Offset<ErrorRes>> for ErrorResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<ErrorRes>;

            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<ErrorRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAsOptional<::planus::Offset<ErrorRes>> for ErrorResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<ErrorRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<ErrorRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAsOffset<ErrorRes> for ErrorResBuilder<(T0,)>
        {
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<ErrorRes> {
                let (v0,) = &self.0;
                ErrorRes::create(builder, v0)
            }
        }

        /// Reference to a deserialized [ErrorRes].
        #[derive(Copy, Clone)]
        pub struct ErrorResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> ErrorResRef<'a> {
            /// Getter for the [`error` field](ErrorRes#structfield.error).
            #[inline]
            pub fn error(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "ErrorRes", "error")
            }
        }

        impl<'a> ::core::fmt::Debug for ErrorResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("ErrorResRef");
                if let ::core::option::Option::Some(field_error) = self.error().transpose() {
                    f.field("error", &field_error);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<ErrorResRef<'a>> for ErrorRes {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: ErrorResRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    error: if let ::core::option::Option::Some(error) = value.error()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(error)?)
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for ErrorResRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for ErrorResRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location("[ErrorResRef]", "get", buffer.offset_from_start)
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<ErrorRes>> for ErrorRes {
            type Value = ::planus::Offset<ErrorRes>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<ErrorRes>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for ErrorResRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[ErrorResRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `RegisterNumberReq` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `RegisterNumberReq` in the file `auth/auth.fbs:9`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct RegisterNumberReq {
            /// The field `name` in the table `RegisterNumberReq`
            pub name: ::core::option::Option<::planus::alloc::string::String>,
            /// The field `number` in the table `RegisterNumberReq`
            pub number: ::core::option::Option<::planus::alloc::string::String>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for RegisterNumberReq {
            fn default() -> Self {
                Self {
                    name: ::core::default::Default::default(),
                    number: ::core::default::Default::default(),
                }
            }
        }

        impl RegisterNumberReq {
            /// Creates a [RegisterNumberReqBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> RegisterNumberReqBuilder<()> {
                RegisterNumberReqBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_number: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            ) -> ::planus::Offset<Self> {
                let prepared_name = field_name.prepare(builder);
                let prepared_number = field_number.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<8> =
                    ::core::default::Default::default();
                if prepared_name.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }
                if prepared_number.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(1);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_name) = prepared_name {
                            object_writer.write::<_, _, 4>(&prepared_name);
                        }
                        if let ::core::option::Option::Some(prepared_number) = prepared_number {
                            object_writer.write::<_, _, 4>(&prepared_number);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<RegisterNumberReq>> for RegisterNumberReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<RegisterNumberReq>> for RegisterNumberReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<RegisterNumberReq> for RegisterNumberReq {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                RegisterNumberReq::create(builder, &self.name, &self.number)
            }
        }

        /// Builder for serializing an instance of the [RegisterNumberReq] type.
        ///
        /// Can be created using the [RegisterNumberReq::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct RegisterNumberReqBuilder<State>(State);

        impl RegisterNumberReqBuilder<()> {
            /// Setter for the [`name` field](RegisterNumberReq#structfield.name).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name<T0>(self, value: T0) -> RegisterNumberReqBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                RegisterNumberReqBuilder((value,))
            }

            /// Sets the [`name` field](RegisterNumberReq#structfield.name) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name_as_null(self) -> RegisterNumberReqBuilder<((),)> {
                self.name(())
            }
        }

        impl<T0> RegisterNumberReqBuilder<(T0,)> {
            /// Setter for the [`number` field](RegisterNumberReq#structfield.number).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn number<T1>(self, value: T1) -> RegisterNumberReqBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                let (v0,) = self.0;
                RegisterNumberReqBuilder((v0, value))
            }

            /// Sets the [`number` field](RegisterNumberReq#structfield.number) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn number_as_null(self) -> RegisterNumberReqBuilder<(T0, ())> {
                self.number(())
            }
        }

        impl<T0, T1> RegisterNumberReqBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RegisterNumberReq].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq>
            where
                Self: ::planus::WriteAsOffset<RegisterNumberReq>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            > ::planus::WriteAs<::planus::Offset<RegisterNumberReq>>
            for RegisterNumberReqBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<RegisterNumberReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            > ::planus::WriteAsOptional<::planus::Offset<RegisterNumberReq>>
            for RegisterNumberReqBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<RegisterNumberReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            > ::planus::WriteAsOffset<RegisterNumberReq> for RegisterNumberReqBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                let (v0, v1) = &self.0;
                RegisterNumberReq::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [RegisterNumberReq].
        #[derive(Copy, Clone)]
        pub struct RegisterNumberReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> RegisterNumberReqRef<'a> {
            /// Getter for the [`name` field](RegisterNumberReq#structfield.name).
            #[inline]
            pub fn name(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "RegisterNumberReq", "name")
            }

            /// Getter for the [`number` field](RegisterNumberReq#structfield.number).
            #[inline]
            pub fn number(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(1, "RegisterNumberReq", "number")
            }
        }

        impl<'a> ::core::fmt::Debug for RegisterNumberReqRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("RegisterNumberReqRef");
                if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                    f.field("name", &field_name);
                }
                if let ::core::option::Option::Some(field_number) = self.number().transpose() {
                    f.field("number", &field_number);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<RegisterNumberReqRef<'a>> for RegisterNumberReq {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: RegisterNumberReqRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    name: if let ::core::option::Option::Some(name) = value.name()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(name)?)
                    } else {
                        ::core::option::Option::None
                    },
                    number: if let ::core::option::Option::Some(number) = value.number()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(number)?)
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for RegisterNumberReqRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for RegisterNumberReqRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[RegisterNumberReqRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<RegisterNumberReq>> for RegisterNumberReq {
            type Value = ::planus::Offset<RegisterNumberReq>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<RegisterNumberReq>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for RegisterNumberReqRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[RegisterNumberReqRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `RegisterNumberRes` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `RegisterNumberRes` in the file `auth/auth.fbs:14`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct RegisterNumberRes {
            /// The field `multiplier` in the table `RegisterNumberRes`
            pub multiplier: u16,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for RegisterNumberRes {
            fn default() -> Self {
                Self { multiplier: 0 }
            }
        }

        impl RegisterNumberRes {
            /// Creates a [RegisterNumberResBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> RegisterNumberResBuilder<()> {
                RegisterNumberResBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_multiplier: impl ::planus::WriteAsDefault<u16, u16>,
            ) -> ::planus::Offset<Self> {
                let prepared_multiplier = field_multiplier.prepare(builder, &0);

                let mut table_writer: ::planus::table_writer::TableWriter<6> =
                    ::core::default::Default::default();
                if prepared_multiplier.is_some() {
                    table_writer.write_entry::<u16>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_multiplier) =
                            prepared_multiplier
                        {
                            object_writer.write::<_, _, 2>(&prepared_multiplier);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<RegisterNumberRes>> for RegisterNumberRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<RegisterNumberRes>> for RegisterNumberRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<RegisterNumberRes> for RegisterNumberRes {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                RegisterNumberRes::create(builder, self.multiplier)
            }
        }

        /// Builder for serializing an instance of the [RegisterNumberRes] type.
        ///
        /// Can be created using the [RegisterNumberRes::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct RegisterNumberResBuilder<State>(State);

        impl RegisterNumberResBuilder<()> {
            /// Setter for the [`multiplier` field](RegisterNumberRes#structfield.multiplier).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn multiplier<T0>(self, value: T0) -> RegisterNumberResBuilder<(T0,)>
            where
                T0: ::planus::WriteAsDefault<u16, u16>,
            {
                RegisterNumberResBuilder((value,))
            }

            /// Sets the [`multiplier` field](RegisterNumberRes#structfield.multiplier) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn multiplier_as_default(
                self,
            ) -> RegisterNumberResBuilder<(::planus::DefaultValue,)> {
                self.multiplier(::planus::DefaultValue)
            }
        }

        impl<T0> RegisterNumberResBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [RegisterNumberRes].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes>
            where
                Self: ::planus::WriteAsOffset<RegisterNumberRes>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<u16, u16>>
            ::planus::WriteAs<::planus::Offset<RegisterNumberRes>>
            for RegisterNumberResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<RegisterNumberRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsDefault<u16, u16>>
            ::planus::WriteAsOptional<::planus::Offset<RegisterNumberRes>>
            for RegisterNumberResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<RegisterNumberRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsDefault<u16, u16>> ::planus::WriteAsOffset<RegisterNumberRes>
            for RegisterNumberResBuilder<(T0,)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                let (v0,) = &self.0;
                RegisterNumberRes::create(builder, v0)
            }
        }

        /// Reference to a deserialized [RegisterNumberRes].
        #[derive(Copy, Clone)]
        pub struct RegisterNumberResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> RegisterNumberResRef<'a> {
            /// Getter for the [`multiplier` field](RegisterNumberRes#structfield.multiplier).
            #[inline]
            pub fn multiplier(&self) -> ::planus::Result<u16> {
                ::core::result::Result::Ok(
                    self.0
                        .access(0, "RegisterNumberRes", "multiplier")?
                        .unwrap_or(0),
                )
            }
        }

        impl<'a> ::core::fmt::Debug for RegisterNumberResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("RegisterNumberResRef");
                f.field("multiplier", &self.multiplier());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<RegisterNumberResRef<'a>> for RegisterNumberRes {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: RegisterNumberResRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    multiplier: ::core::convert::TryInto::try_into(value.multiplier()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for RegisterNumberResRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for RegisterNumberResRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[RegisterNumberResRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<RegisterNumberRes>> for RegisterNumberRes {
            type Value = ::planus::Offset<RegisterNumberRes>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<RegisterNumberRes>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for RegisterNumberResRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[RegisterNumberResRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `VerifyNumberReq` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `VerifyNumberReq` in the file `auth/auth.fbs:18`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct VerifyNumberReq {
            /// The field `number` in the table `VerifyNumberReq`
            pub number: ::core::option::Option<::planus::alloc::string::String>,
            /// The field `code` in the table `VerifyNumberReq`
            pub code: u16,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyNumberReq {
            fn default() -> Self {
                Self {
                    number: ::core::default::Default::default(),
                    code: 0,
                }
            }
        }

        impl VerifyNumberReq {
            /// Creates a [VerifyNumberReqBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> VerifyNumberReqBuilder<()> {
                VerifyNumberReqBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_number: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_code: impl ::planus::WriteAsDefault<u16, u16>,
            ) -> ::planus::Offset<Self> {
                let prepared_number = field_number.prepare(builder);
                let prepared_code = field_code.prepare(builder, &0);

                let mut table_writer: ::planus::table_writer::TableWriter<8> =
                    ::core::default::Default::default();
                if prepared_number.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }
                if prepared_code.is_some() {
                    table_writer.write_entry::<u16>(1);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_number) = prepared_number {
                            object_writer.write::<_, _, 4>(&prepared_number);
                        }
                        if let ::core::option::Option::Some(prepared_code) = prepared_code {
                            object_writer.write::<_, _, 2>(&prepared_code);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyNumberReq>> for VerifyNumberReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyNumberReq>> for VerifyNumberReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyNumberReq> for VerifyNumberReq {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                VerifyNumberReq::create(builder, &self.number, self.code)
            }
        }

        /// Builder for serializing an instance of the [VerifyNumberReq] type.
        ///
        /// Can be created using the [VerifyNumberReq::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct VerifyNumberReqBuilder<State>(State);

        impl VerifyNumberReqBuilder<()> {
            /// Setter for the [`number` field](VerifyNumberReq#structfield.number).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn number<T0>(self, value: T0) -> VerifyNumberReqBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                VerifyNumberReqBuilder((value,))
            }

            /// Sets the [`number` field](VerifyNumberReq#structfield.number) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn number_as_null(self) -> VerifyNumberReqBuilder<((),)> {
                self.number(())
            }
        }

        impl<T0> VerifyNumberReqBuilder<(T0,)> {
            /// Setter for the [`code` field](VerifyNumberReq#structfield.code).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn code<T1>(self, value: T1) -> VerifyNumberReqBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<u16, u16>,
            {
                let (v0,) = self.0;
                VerifyNumberReqBuilder((v0, value))
            }

            /// Sets the [`code` field](VerifyNumberReq#structfield.code) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn code_as_default(self) -> VerifyNumberReqBuilder<(T0, ::planus::DefaultValue)> {
                self.code(::planus::DefaultValue)
            }
        }

        impl<T0, T1> VerifyNumberReqBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [VerifyNumberReq].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq>
            where
                Self: ::planus::WriteAsOffset<VerifyNumberReq>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u16, u16>,
            > ::planus::WriteAs<::planus::Offset<VerifyNumberReq>>
            for VerifyNumberReqBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<VerifyNumberReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u16, u16>,
            > ::planus::WriteAsOptional<::planus::Offset<VerifyNumberReq>>
            for VerifyNumberReqBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<VerifyNumberReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u16, u16>,
            > ::planus::WriteAsOffset<VerifyNumberReq> for VerifyNumberReqBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                let (v0, v1) = &self.0;
                VerifyNumberReq::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [VerifyNumberReq].
        #[derive(Copy, Clone)]
        pub struct VerifyNumberReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyNumberReqRef<'a> {
            /// Getter for the [`number` field](VerifyNumberReq#structfield.number).
            #[inline]
            pub fn number(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "VerifyNumberReq", "number")
            }

            /// Getter for the [`code` field](VerifyNumberReq#structfield.code).
            #[inline]
            pub fn code(&self) -> ::planus::Result<u16> {
                ::core::result::Result::Ok(
                    self.0.access(1, "VerifyNumberReq", "code")?.unwrap_or(0),
                )
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyNumberReqRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyNumberReqRef");
                if let ::core::option::Option::Some(field_number) = self.number().transpose() {
                    f.field("number", &field_number);
                }
                f.field("code", &self.code());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<VerifyNumberReqRef<'a>> for VerifyNumberReq {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: VerifyNumberReqRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    number: if let ::core::option::Option::Some(number) = value.number()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(number)?)
                    } else {
                        ::core::option::Option::None
                    },
                    code: ::core::convert::TryInto::try_into(value.code()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for VerifyNumberReqRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for VerifyNumberReqRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[VerifyNumberReqRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<VerifyNumberReq>> for VerifyNumberReq {
            type Value = ::planus::Offset<VerifyNumberReq>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<VerifyNumberReq>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for VerifyNumberReqRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[VerifyNumberReqRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `VerifyNumberRes` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `VerifyNumberRes` in the file `auth/auth.fbs:23`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct VerifyNumberRes {
            /// The field `options` in the table `VerifyNumberRes`
            pub options: ::core::option::Option<
                ::planus::alloc::boxed::Box<self::web_authn::CredentialCreationOptions>,
            >,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyNumberRes {
            fn default() -> Self {
                Self {
                    options: ::core::default::Default::default(),
                }
            }
        }

        impl VerifyNumberRes {
            /// Creates a [VerifyNumberResBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> VerifyNumberResBuilder<()> {
                VerifyNumberResBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_options: impl ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            ) -> ::planus::Offset<Self> {
                let prepared_options = field_options.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<6> =
                    ::core::default::Default::default();
                if prepared_options.is_some() {
                    table_writer.write_entry::<::planus::Offset<self::web_authn::CredentialCreationOptions>>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_options) = prepared_options {
                            object_writer.write::<_, _, 4>(&prepared_options);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyNumberRes>> for VerifyNumberRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyNumberRes>> for VerifyNumberRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyNumberRes> for VerifyNumberRes {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                VerifyNumberRes::create(builder, &self.options)
            }
        }

        /// Builder for serializing an instance of the [VerifyNumberRes] type.
        ///
        /// Can be created using the [VerifyNumberRes::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct VerifyNumberResBuilder<State>(State);

        impl VerifyNumberResBuilder<()> {
            /// Setter for the [`options` field](VerifyNumberRes#structfield.options).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn options<T0>(self, value: T0) -> VerifyNumberResBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            {
                VerifyNumberResBuilder((value,))
            }

            /// Sets the [`options` field](VerifyNumberRes#structfield.options) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn options_as_null(self) -> VerifyNumberResBuilder<((),)> {
                self.options(())
            }
        }

        impl<T0> VerifyNumberResBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [VerifyNumberRes].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes>
            where
                Self: ::planus::WriteAsOffset<VerifyNumberRes>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            > ::planus::WriteAs<::planus::Offset<VerifyNumberRes>>
            for VerifyNumberResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<VerifyNumberRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            > ::planus::WriteAsOptional<::planus::Offset<VerifyNumberRes>>
            for VerifyNumberResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<VerifyNumberRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            > ::planus::WriteAsOffset<VerifyNumberRes> for VerifyNumberResBuilder<(T0,)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                let (v0,) = &self.0;
                VerifyNumberRes::create(builder, v0)
            }
        }

        /// Reference to a deserialized [VerifyNumberRes].
        #[derive(Copy, Clone)]
        pub struct VerifyNumberResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyNumberResRef<'a> {
            /// Getter for the [`options` field](VerifyNumberRes#structfield.options).
            #[inline]
            pub fn options(
                &self,
            ) -> ::planus::Result<
                ::core::option::Option<self::web_authn::CredentialCreationOptionsRef<'a>>,
            > {
                self.0.access(0, "VerifyNumberRes", "options")
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyNumberResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyNumberResRef");
                if let ::core::option::Option::Some(field_options) = self.options().transpose() {
                    f.field("options", &field_options);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<VerifyNumberResRef<'a>> for VerifyNumberRes {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: VerifyNumberResRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    options: if let ::core::option::Option::Some(options) = value.options()? {
                        ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                            ::core::convert::TryInto::try_into(options)?,
                        ))
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for VerifyNumberResRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for VerifyNumberResRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[VerifyNumberResRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<VerifyNumberRes>> for VerifyNumberRes {
            type Value = ::planus::Offset<VerifyNumberRes>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<VerifyNumberRes>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for VerifyNumberResRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[VerifyNumberResRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `VerifyWebAuthnReq` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `VerifyWebAuthnReq` in the file `auth/auth.fbs:27`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct VerifyWebAuthnReq {
            /// The field `name` in the table `VerifyWebAuthnReq`
            pub name: ::core::option::Option<::planus::alloc::string::String>,
            /// The field `id` in the table `VerifyWebAuthnReq`
            pub id: ::core::option::Option<::planus::alloc::string::String>,
            /// The field `webauthn` in the table `VerifyWebAuthnReq`
            pub webauthn: ::core::option::Option<
                ::planus::alloc::boxed::Box<self::web_authn::RegisterPublicKeyCredential>,
            >,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyWebAuthnReq {
            fn default() -> Self {
                Self {
                    name: ::core::default::Default::default(),
                    id: ::core::default::Default::default(),
                    webauthn: ::core::default::Default::default(),
                }
            }
        }

        impl VerifyWebAuthnReq {
            /// Creates a [VerifyWebAuthnReqBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> VerifyWebAuthnReqBuilder<()> {
                VerifyWebAuthnReqBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_id: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_webauthn: impl ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::RegisterPublicKeyCredential>,
                >,
            ) -> ::planus::Offset<Self> {
                let prepared_name = field_name.prepare(builder);
                let prepared_id = field_id.prepare(builder);
                let prepared_webauthn = field_webauthn.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<10> =
                    ::core::default::Default::default();
                if prepared_name.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }
                if prepared_id.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(1);
                }
                if prepared_webauthn.is_some() {
                    table_writer.write_entry::<::planus::Offset<self::web_authn::RegisterPublicKeyCredential>>(2);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_name) = prepared_name {
                            object_writer.write::<_, _, 4>(&prepared_name);
                        }
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            object_writer.write::<_, _, 4>(&prepared_id);
                        }
                        if let ::core::option::Option::Some(prepared_webauthn) = prepared_webauthn {
                            object_writer.write::<_, _, 4>(&prepared_webauthn);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyWebAuthnReq>> for VerifyWebAuthnReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnReq>> for VerifyWebAuthnReq {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyWebAuthnReq> for VerifyWebAuthnReq {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                VerifyWebAuthnReq::create(builder, &self.name, &self.id, &self.webauthn)
            }
        }

        /// Builder for serializing an instance of the [VerifyWebAuthnReq] type.
        ///
        /// Can be created using the [VerifyWebAuthnReq::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct VerifyWebAuthnReqBuilder<State>(State);

        impl VerifyWebAuthnReqBuilder<()> {
            /// Setter for the [`name` field](VerifyWebAuthnReq#structfield.name).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name<T0>(self, value: T0) -> VerifyWebAuthnReqBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                VerifyWebAuthnReqBuilder((value,))
            }

            /// Sets the [`name` field](VerifyWebAuthnReq#structfield.name) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name_as_null(self) -> VerifyWebAuthnReqBuilder<((),)> {
                self.name(())
            }
        }

        impl<T0> VerifyWebAuthnReqBuilder<(T0,)> {
            /// Setter for the [`id` field](VerifyWebAuthnReq#structfield.id).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id<T1>(self, value: T1) -> VerifyWebAuthnReqBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                let (v0,) = self.0;
                VerifyWebAuthnReqBuilder((v0, value))
            }

            /// Sets the [`id` field](VerifyWebAuthnReq#structfield.id) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn id_as_null(self) -> VerifyWebAuthnReqBuilder<(T0, ())> {
                self.id(())
            }
        }

        impl<T0, T1> VerifyWebAuthnReqBuilder<(T0, T1)> {
            /// Setter for the [`webauthn` field](VerifyWebAuthnReq#structfield.webauthn).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn webauthn<T2>(self, value: T2) -> VerifyWebAuthnReqBuilder<(T0, T1, T2)>
            where
                T2: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::RegisterPublicKeyCredential>,
                >,
            {
                let (v0, v1) = self.0;
                VerifyWebAuthnReqBuilder((v0, v1, value))
            }

            /// Sets the [`webauthn` field](VerifyWebAuthnReq#structfield.webauthn) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn webauthn_as_null(self) -> VerifyWebAuthnReqBuilder<(T0, T1, ())> {
                self.webauthn(())
            }
        }

        impl<T0, T1, T2> VerifyWebAuthnReqBuilder<(T0, T1, T2)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [VerifyWebAuthnReq].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq>
            where
                Self: ::planus::WriteAsOffset<VerifyWebAuthnReq>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T2: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::RegisterPublicKeyCredential>,
                >,
            > ::planus::WriteAs<::planus::Offset<VerifyWebAuthnReq>>
            for VerifyWebAuthnReqBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<VerifyWebAuthnReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T2: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::RegisterPublicKeyCredential>,
                >,
            > ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnReq>>
            for VerifyWebAuthnReqBuilder<(T0, T1, T2)>
        {
            type Prepared = ::planus::Offset<VerifyWebAuthnReq>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T2: ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::RegisterPublicKeyCredential>,
                >,
            > ::planus::WriteAsOffset<VerifyWebAuthnReq>
            for VerifyWebAuthnReqBuilder<(T0, T1, T2)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                let (v0, v1, v2) = &self.0;
                VerifyWebAuthnReq::create(builder, v0, v1, v2)
            }
        }

        /// Reference to a deserialized [VerifyWebAuthnReq].
        #[derive(Copy, Clone)]
        pub struct VerifyWebAuthnReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyWebAuthnReqRef<'a> {
            /// Getter for the [`name` field](VerifyWebAuthnReq#structfield.name).
            #[inline]
            pub fn name(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "VerifyWebAuthnReq", "name")
            }

            /// Getter for the [`id` field](VerifyWebAuthnReq#structfield.id).
            #[inline]
            pub fn id(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(1, "VerifyWebAuthnReq", "id")
            }

            /// Getter for the [`webauthn` field](VerifyWebAuthnReq#structfield.webauthn).
            #[inline]
            pub fn webauthn(
                &self,
            ) -> ::planus::Result<
                ::core::option::Option<self::web_authn::RegisterPublicKeyCredentialRef<'a>>,
            > {
                self.0.access(2, "VerifyWebAuthnReq", "webauthn")
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyWebAuthnReqRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyWebAuthnReqRef");
                if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                    f.field("name", &field_name);
                }
                if let ::core::option::Option::Some(field_id) = self.id().transpose() {
                    f.field("id", &field_id);
                }
                if let ::core::option::Option::Some(field_webauthn) = self.webauthn().transpose() {
                    f.field("webauthn", &field_webauthn);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<VerifyWebAuthnReqRef<'a>> for VerifyWebAuthnReq {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: VerifyWebAuthnReqRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    name: if let ::core::option::Option::Some(name) = value.name()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(name)?)
                    } else {
                        ::core::option::Option::None
                    },
                    id: if let ::core::option::Option::Some(id) = value.id()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(id)?)
                    } else {
                        ::core::option::Option::None
                    },
                    webauthn: if let ::core::option::Option::Some(webauthn) = value.webauthn()? {
                        ::core::option::Option::Some(::planus::alloc::boxed::Box::new(
                            ::core::convert::TryInto::try_into(webauthn)?,
                        ))
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for VerifyWebAuthnReqRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for VerifyWebAuthnReqRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[VerifyWebAuthnReqRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<VerifyWebAuthnReq>> for VerifyWebAuthnReq {
            type Value = ::planus::Offset<VerifyWebAuthnReq>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<VerifyWebAuthnReq>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for VerifyWebAuthnReqRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[VerifyWebAuthnReqRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `VerifyWebAuthnRes` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `VerifyWebAuthnRes` in the file `auth/auth.fbs:33`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct VerifyWebAuthnRes {
            /// The field `token` in the table `VerifyWebAuthnRes`
            pub token: ::core::option::Option<::planus::alloc::string::String>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyWebAuthnRes {
            fn default() -> Self {
                Self {
                    token: ::core::default::Default::default(),
                }
            }
        }

        impl VerifyWebAuthnRes {
            /// Creates a [VerifyWebAuthnResBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> VerifyWebAuthnResBuilder<()> {
                VerifyWebAuthnResBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_token: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            ) -> ::planus::Offset<Self> {
                let prepared_token = field_token.prepare(builder);

                let mut table_writer: ::planus::table_writer::TableWriter<6> =
                    ::core::default::Default::default();
                if prepared_token.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_token) = prepared_token {
                            object_writer.write::<_, _, 4>(&prepared_token);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyWebAuthnRes>> for VerifyWebAuthnRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnRes>> for VerifyWebAuthnRes {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyWebAuthnRes> for VerifyWebAuthnRes {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                VerifyWebAuthnRes::create(builder, &self.token)
            }
        }

        /// Builder for serializing an instance of the [VerifyWebAuthnRes] type.
        ///
        /// Can be created using the [VerifyWebAuthnRes::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct VerifyWebAuthnResBuilder<State>(State);

        impl VerifyWebAuthnResBuilder<()> {
            /// Setter for the [`token` field](VerifyWebAuthnRes#structfield.token).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn token<T0>(self, value: T0) -> VerifyWebAuthnResBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                VerifyWebAuthnResBuilder((value,))
            }

            /// Sets the [`token` field](VerifyWebAuthnRes#structfield.token) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn token_as_null(self) -> VerifyWebAuthnResBuilder<((),)> {
                self.token(())
            }
        }

        impl<T0> VerifyWebAuthnResBuilder<(T0,)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [VerifyWebAuthnRes].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes>
            where
                Self: ::planus::WriteAsOffset<VerifyWebAuthnRes>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAs<::planus::Offset<VerifyWebAuthnRes>>
            for VerifyWebAuthnResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<VerifyWebAuthnRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnRes>>
            for VerifyWebAuthnResBuilder<(T0,)>
        {
            type Prepared = ::planus::Offset<VerifyWebAuthnRes>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>>
            ::planus::WriteAsOffset<VerifyWebAuthnRes> for VerifyWebAuthnResBuilder<(T0,)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                let (v0,) = &self.0;
                VerifyWebAuthnRes::create(builder, v0)
            }
        }

        /// Reference to a deserialized [VerifyWebAuthnRes].
        #[derive(Copy, Clone)]
        pub struct VerifyWebAuthnResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyWebAuthnResRef<'a> {
            /// Getter for the [`token` field](VerifyWebAuthnRes#structfield.token).
            #[inline]
            pub fn token(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "VerifyWebAuthnRes", "token")
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyWebAuthnResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyWebAuthnResRef");
                if let ::core::option::Option::Some(field_token) = self.token().transpose() {
                    f.field("token", &field_token);
                }
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<VerifyWebAuthnResRef<'a>> for VerifyWebAuthnRes {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: VerifyWebAuthnResRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    token: if let ::core::option::Option::Some(token) = value.token()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(token)?)
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for VerifyWebAuthnResRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for VerifyWebAuthnResRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[VerifyWebAuthnResRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<VerifyWebAuthnRes>> for VerifyWebAuthnRes {
            type Value = ::planus::Offset<VerifyWebAuthnRes>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<VerifyWebAuthnRes>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for VerifyWebAuthnResRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[VerifyWebAuthnResRef]", "read_as_root", 0)
                })
            }
        }

        /// The table `NumberRegistration` in the namespace `Auth`
        ///
        /// Generated from these locations:
        /// * Table `NumberRegistration` in the file `auth/auth.fbs:37`
        #[derive(
            Clone,
            Debug,
            PartialEq,
            PartialOrd,
            Eq,
            Ord,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        pub struct NumberRegistration {
            /// The field `name` in the table `NumberRegistration`
            pub name: ::core::option::Option<::planus::alloc::string::String>,
            /// The field `code` in the table `NumberRegistration`
            pub code: u64,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for NumberRegistration {
            fn default() -> Self {
                Self {
                    name: ::core::default::Default::default(),
                    code: 0,
                }
            }
        }

        impl NumberRegistration {
            /// Creates a [NumberRegistrationBuilder] for serializing an instance of this table.
            #[inline]
            pub fn builder() -> NumberRegistrationBuilder<()> {
                NumberRegistrationBuilder(())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_code: impl ::planus::WriteAsDefault<u64, u64>,
            ) -> ::planus::Offset<Self> {
                let prepared_name = field_name.prepare(builder);
                let prepared_code = field_code.prepare(builder, &0);

                let mut table_writer: ::planus::table_writer::TableWriter<8> =
                    ::core::default::Default::default();
                if prepared_code.is_some() {
                    table_writer.write_entry::<u64>(1);
                }
                if prepared_name.is_some() {
                    table_writer.write_entry::<::planus::Offset<str>>(0);
                }

                unsafe {
                    table_writer.finish(builder, |object_writer| {
                        if let ::core::option::Option::Some(prepared_code) = prepared_code {
                            object_writer.write::<_, _, 8>(&prepared_code);
                        }
                        if let ::core::option::Option::Some(prepared_name) = prepared_name {
                            object_writer.write::<_, _, 4>(&prepared_name);
                        }
                    });
                }
                builder.current_offset()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<NumberRegistration>> for NumberRegistration {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<NumberRegistration> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<NumberRegistration>> for NumberRegistration {
            type Prepared = ::planus::Offset<Self>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<NumberRegistration>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<NumberRegistration> for NumberRegistration {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<NumberRegistration> {
                NumberRegistration::create(builder, &self.name, self.code)
            }
        }

        /// Builder for serializing an instance of the [NumberRegistration] type.
        ///
        /// Can be created using the [NumberRegistration::builder] method.
        #[derive(Debug)]
        #[must_use]
        pub struct NumberRegistrationBuilder<State>(State);

        impl NumberRegistrationBuilder<()> {
            /// Setter for the [`name` field](NumberRegistration#structfield.name).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name<T0>(self, value: T0) -> NumberRegistrationBuilder<(T0,)>
            where
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            {
                NumberRegistrationBuilder((value,))
            }

            /// Sets the [`name` field](NumberRegistration#structfield.name) to null.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn name_as_null(self) -> NumberRegistrationBuilder<((),)> {
                self.name(())
            }
        }

        impl<T0> NumberRegistrationBuilder<(T0,)> {
            /// Setter for the [`code` field](NumberRegistration#structfield.code).
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn code<T1>(self, value: T1) -> NumberRegistrationBuilder<(T0, T1)>
            where
                T1: ::planus::WriteAsDefault<u64, u64>,
            {
                let (v0,) = self.0;
                NumberRegistrationBuilder((v0, value))
            }

            /// Sets the [`code` field](NumberRegistration#structfield.code) to the default value.
            #[inline]
            #[allow(clippy::type_complexity)]
            pub fn code_as_default(
                self,
            ) -> NumberRegistrationBuilder<(T0, ::planus::DefaultValue)> {
                self.code(::planus::DefaultValue)
            }
        }

        impl<T0, T1> NumberRegistrationBuilder<(T0, T1)> {
            /// Finish writing the builder to get an [Offset](::planus::Offset) to a serialized [NumberRegistration].
            #[inline]
            pub fn finish(
                self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<NumberRegistration>
            where
                Self: ::planus::WriteAsOffset<NumberRegistration>,
            {
                ::planus::WriteAsOffset::prepare(&self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u64, u64>,
            > ::planus::WriteAs<::planus::Offset<NumberRegistration>>
            for NumberRegistrationBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<NumberRegistration>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<NumberRegistration> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u64, u64>,
            > ::planus::WriteAsOptional<::planus::Offset<NumberRegistration>>
            for NumberRegistrationBuilder<(T0, T1)>
        {
            type Prepared = ::planus::Offset<NumberRegistration>;

            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<NumberRegistration>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl<
                T0: ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                T1: ::planus::WriteAsDefault<u64, u64>,
            > ::planus::WriteAsOffset<NumberRegistration> for NumberRegistrationBuilder<(T0, T1)>
        {
            #[inline]
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<NumberRegistration> {
                let (v0, v1) = &self.0;
                NumberRegistration::create(builder, v0, v1)
            }
        }

        /// Reference to a deserialized [NumberRegistration].
        #[derive(Copy, Clone)]
        pub struct NumberRegistrationRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> NumberRegistrationRef<'a> {
            /// Getter for the [`name` field](NumberRegistration#structfield.name).
            #[inline]
            pub fn name(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "NumberRegistration", "name")
            }

            /// Getter for the [`code` field](NumberRegistration#structfield.code).
            #[inline]
            pub fn code(&self) -> ::planus::Result<u64> {
                ::core::result::Result::Ok(
                    self.0.access(1, "NumberRegistration", "code")?.unwrap_or(0),
                )
            }
        }

        impl<'a> ::core::fmt::Debug for NumberRegistrationRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("NumberRegistrationRef");
                if let ::core::option::Option::Some(field_name) = self.name().transpose() {
                    f.field("name", &field_name);
                }
                f.field("code", &self.code());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<NumberRegistrationRef<'a>> for NumberRegistration {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: NumberRegistrationRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    name: if let ::core::option::Option::Some(name) = value.name()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(name)?)
                    } else {
                        ::core::option::Option::None
                    },
                    code: ::core::convert::TryInto::try_into(value.code()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for NumberRegistrationRef<'a> {
            #[inline]
            fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::core::result::Result<Self, ::planus::errors::ErrorKind> {
                ::core::result::Result::Ok(Self(::planus::table_reader::Table::from_buffer(
                    buffer, offset,
                )?))
            }
        }

        impl<'a> ::planus::VectorReadInner<'a> for NumberRegistrationRef<'a> {
            type Error = ::planus::Error;
            const STRIDE: usize = 4;

            unsafe fn from_buffer(
                buffer: ::planus::SliceWithStartOffset<'a>,
                offset: usize,
            ) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(buffer, offset).map_err(|error_kind| {
                    error_kind.with_error_location(
                        "[NumberRegistrationRef]",
                        "get",
                        buffer.offset_from_start,
                    )
                })
            }
        }

        impl ::planus::VectorWrite<::planus::Offset<NumberRegistration>> for NumberRegistration {
            type Value = ::planus::Offset<NumberRegistration>;
            const STRIDE: usize = 4;
            #[inline]
            fn prepare(&self, builder: &mut ::planus::Builder) -> Self::Value {
                ::planus::WriteAs::prepare(self, builder)
            }

            #[inline]
            unsafe fn write_values(
                values: &[::planus::Offset<NumberRegistration>],
                bytes: *mut ::core::mem::MaybeUninit<u8>,
                buffer_position: u32,
            ) {
                let bytes = bytes as *mut [::core::mem::MaybeUninit<u8>; 4];
                for (i, v) in ::core::iter::Iterator::enumerate(values.iter()) {
                    ::planus::WriteAsPrimitive::write(
                        v,
                        ::planus::Cursor::new(&mut *bytes.add(i)),
                        buffer_position - (Self::STRIDE * i) as u32,
                    );
                }
            }
        }

        impl<'a> ::planus::ReadAsRoot<'a> for NumberRegistrationRef<'a> {
            fn read_as_root(slice: &'a [u8]) -> ::planus::Result<Self> {
                ::planus::TableRead::from_buffer(
                    ::planus::SliceWithStartOffset {
                        buffer: slice,
                        offset_from_start: 0,
                    },
                    0,
                )
                .map_err(|error_kind| {
                    error_kind.with_error_location("[NumberRegistrationRef]", "read_as_root", 0)
                })
            }
        }
    }
}
