pub use root::*;

const _: () = ::planus::check_version_compatibility("planus-0.3.1");

#[no_implicit_prelude]
mod root {
    pub mod auth {
        pub mod web_authn {
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
                pub rp: ::core::option::Option<::planus::alloc::boxed::Box<self::RelyingParty>>,
                pub user: ::core::option::Option<::planus::alloc::boxed::Box<self::User>>,
                pub challenge: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                pub pub_key_cred_params:
                    ::core::option::Option<::planus::alloc::vec::Vec<self::PubKeyCredParams>>,
                pub timeout: u32,
                pub attestation: self::AttestationConveyancePreference,
                pub exclude_credentials: ::core::option::Option<
                    ::planus::alloc::vec::Vec<self::PublicKeyCredentialDescriptor>,
                >,
                pub authenticator_selection: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::AuthenticatorSelectionCriteria>,
                >,
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
                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_rp: impl ::planus::WriteAsOptional<::planus::Offset<self::RelyingParty>>,
                    field_user: impl ::planus::WriteAsOptional<::planus::Offset<self::User>>,
                    field_challenge: impl ::planus::WriteAsOptional<::planus::Offset<[u8]>>,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<20, 33>::new(builder);

                    if prepared_rp.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::RelyingParty>>(2);
                    }
                    if prepared_user.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::User>>(4);
                    }
                    if prepared_challenge.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(6);
                    }
                    if prepared_pub_key_cred_params.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[::planus::Offset<self::PubKeyCredParams>]>>(8);
                    }
                    if prepared_timeout.is_some() {
                        table_writer.calculate_size::<u32>(10);
                    }
                    if prepared_attestation.is_some() {
                        table_writer.calculate_size::<self::AttestationConveyancePreference>(12);
                    }
                    if prepared_exclude_credentials.is_some() {
                        table_writer.calculate_size::<::planus::Offset<
                            [::planus::Offset<self::PublicKeyCredentialDescriptor>],
                        >>(14);
                    }
                    if prepared_authenticator_selection.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::AuthenticatorSelectionCriteria>>(16);
                    }
                    if prepared_extensions.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::RequestRegistrationExtensions>>(18);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_rp) = prepared_rp {
                            table_writer.write::<_, _, 4>(0, &prepared_rp);
                        }
                        if let ::core::option::Option::Some(prepared_user) = prepared_user {
                            table_writer.write::<_, _, 4>(1, &prepared_user);
                        }
                        if let ::core::option::Option::Some(prepared_challenge) = prepared_challenge
                        {
                            table_writer.write::<_, _, 4>(2, &prepared_challenge);
                        }
                        if let ::core::option::Option::Some(prepared_pub_key_cred_params) =
                            prepared_pub_key_cred_params
                        {
                            table_writer.write::<_, _, 4>(3, &prepared_pub_key_cred_params);
                        }
                        if let ::core::option::Option::Some(prepared_timeout) = prepared_timeout {
                            table_writer.write::<_, _, 4>(4, &prepared_timeout);
                        }
                        if let ::core::option::Option::Some(prepared_exclude_credentials) =
                            prepared_exclude_credentials
                        {
                            table_writer.write::<_, _, 4>(6, &prepared_exclude_credentials);
                        }
                        if let ::core::option::Option::Some(prepared_authenticator_selection) =
                            prepared_authenticator_selection
                        {
                            table_writer.write::<_, _, 4>(7, &prepared_authenticator_selection);
                        }
                        if let ::core::option::Option::Some(prepared_extensions) =
                            prepared_extensions
                        {
                            table_writer.write::<_, _, 4>(8, &prepared_extensions);
                        }
                        if let ::core::option::Option::Some(prepared_attestation) =
                            prepared_attestation
                        {
                            table_writer.write::<_, _, 1>(5, &prepared_attestation);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<CredentialCreationOptions>> for CredentialCreationOptions {
                type Prepared = ::planus::Offset<Self>;

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

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredentialCreationOptions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<CredentialCreationOptions> for CredentialCreationOptions {
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
                        &self.timeout,
                        &self.attestation,
                        &self.exclude_credentials,
                        &self.authenticator_selection,
                        &self.extensions,
                    )
                }
            }

            #[derive(Copy, Clone)]
            pub struct CredentialCreationOptionsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> CredentialCreationOptionsRef<'a> {
                pub fn rp(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::RelyingPartyRef<'a>>>
                {
                    self.0.access(0, "CredentialCreationOptions", "rp")
                }

                pub fn user(&self) -> ::planus::Result<::core::option::Option<self::UserRef<'a>>> {
                    self.0.access(1, "CredentialCreationOptions", "user")
                }

                pub fn challenge(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0.access(2, "CredentialCreationOptions", "challenge")
                }

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

                pub fn timeout(&self) -> ::planus::Result<u32> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(4, "CredentialCreationOptions", "timeout")?
                            .unwrap_or(60000),
                    )
                }

                pub fn attestation(
                    &self,
                ) -> ::planus::Result<self::AttestationConveyancePreference> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(5, "CredentialCreationOptions", "attestation")?
                            .unwrap_or(self::AttestationConveyancePreference::None),
                    )
                }

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

                pub fn authenticator_selection(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::AuthenticatorSelectionCriteriaRef<'a>>,
                > {
                    self.0
                        .access(7, "CredentialCreationOptions", "authenticator_selection")
                }

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
                            ::core::option::Option::Some(challenge.to_vec()?)
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
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                pub raw_id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                pub response: ::core::option::Option<
                    ::planus::alloc::boxed::Box<self::AuthenticatorAttestationResponse>,
                >,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<10, 16>::new(builder);

                    if prepared_id.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(2);
                    }
                    if prepared_raw_id.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(4);
                    }
                    if prepared_response.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::AuthenticatorAttestationResponse>>(6);
                    }
                    if prepared_client_extension_results.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::RegistrationExtensionsClientOutputs>>(8);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            table_writer.write::<_, _, 4>(0, &prepared_id);
                        }
                        if let ::core::option::Option::Some(prepared_raw_id) = prepared_raw_id {
                            table_writer.write::<_, _, 4>(1, &prepared_raw_id);
                        }
                        if let ::core::option::Option::Some(prepared_response) = prepared_response {
                            table_writer.write::<_, _, 4>(2, &prepared_response);
                        }
                        if let ::core::option::Option::Some(prepared_client_extension_results) =
                            prepared_client_extension_results
                        {
                            table_writer.write::<_, _, 4>(3, &prepared_client_extension_results);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RegisterPublicKeyCredential>>
                for RegisterPublicKeyCredential
            {
                type Prepared = ::planus::Offset<Self>;

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

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RegisterPublicKeyCredential>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RegisterPublicKeyCredential> for RegisterPublicKeyCredential {
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

            #[derive(Copy, Clone)]
            pub struct RegisterPublicKeyCredentialRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RegisterPublicKeyCredentialRef<'a> {
                pub fn id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0.access(0, "RegisterPublicKeyCredential", "id")
                }

                pub fn raw_id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0.access(1, "RegisterPublicKeyCredential", "raw_id")
                }

                pub fn response(
                    &self,
                ) -> ::planus::Result<
                    ::core::option::Option<self::AuthenticatorAttestationResponseRef<'a>>,
                > {
                    self.0.access(2, "RegisterPublicKeyCredential", "response")
                }

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
                        id: if let ::core::option::Option::Some(id) = value.id()? {
                            ::core::option::Option::Some(id.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                        raw_id: if let ::core::option::Option::Some(raw_id) = value.raw_id()? {
                            ::core::option::Option::Some(raw_id.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
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
                pub client_data: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                pub attestation_object: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<8, 12>::new(builder);

                    if prepared_client_data.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(2);
                    }
                    if prepared_attestation_object.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(4);
                    }
                    if prepared_transports.is_some() {
                        table_writer
                            .calculate_size::<::planus::Offset<[self::AuthenticatorTransport]>>(6);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_client_data) =
                            prepared_client_data
                        {
                            table_writer.write::<_, _, 4>(0, &prepared_client_data);
                        }
                        if let ::core::option::Option::Some(prepared_attestation_object) =
                            prepared_attestation_object
                        {
                            table_writer.write::<_, _, 4>(1, &prepared_attestation_object);
                        }
                        if let ::core::option::Option::Some(prepared_transports) =
                            prepared_transports
                        {
                            table_writer.write::<_, _, 4>(2, &prepared_transports);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<AuthenticatorAttestationResponse>>
                for AuthenticatorAttestationResponse
            {
                type Prepared = ::planus::Offset<Self>;

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

            #[derive(Copy, Clone)]
            pub struct AuthenticatorAttestationResponseRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> AuthenticatorAttestationResponseRef<'a> {
                pub fn client_data(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0
                        .access(0, "AuthenticatorAttestationResponse", "client_data")
                }

                pub fn attestation_object(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0
                        .access(1, "AuthenticatorAttestationResponse", "attestation_object")
                }

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
                        client_data: if let ::core::option::Option::Some(client_data) =
                            value.client_data()?
                        {
                            ::core::option::Option::Some(client_data.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
                        attestation_object: if let ::core::option::Option::Some(
                            attestation_object,
                        ) = value.attestation_object()?
                        {
                            ::core::option::Option::Some(attestation_object.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
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
                pub app_id: bool,
                pub cred_props: bool,
                pub hmac_secret: bool,
                pub cred_protect: self::CredentialProtectionPolicy,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<12, 8>::new(builder);

                    if prepared_app_id.is_some() {
                        table_writer.calculate_size::<bool>(2);
                    }
                    if prepared_cred_props.is_some() {
                        table_writer.calculate_size::<bool>(4);
                    }
                    if prepared_hmac_secret.is_some() {
                        table_writer.calculate_size::<bool>(6);
                    }
                    if prepared_cred_protect.is_some() {
                        table_writer.calculate_size::<self::CredentialProtectionPolicy>(8);
                    }
                    if prepared_min_pin_length.is_some() {
                        table_writer.calculate_size::<u32>(10);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_min_pin_length) =
                            prepared_min_pin_length
                        {
                            table_writer.write::<_, _, 4>(4, &prepared_min_pin_length);
                        }
                        if let ::core::option::Option::Some(prepared_app_id) = prepared_app_id {
                            table_writer.write::<_, _, 1>(0, &prepared_app_id);
                        }
                        if let ::core::option::Option::Some(prepared_cred_props) =
                            prepared_cred_props
                        {
                            table_writer.write::<_, _, 1>(1, &prepared_cred_props);
                        }
                        if let ::core::option::Option::Some(prepared_hmac_secret) =
                            prepared_hmac_secret
                        {
                            table_writer.write::<_, _, 1>(2, &prepared_hmac_secret);
                        }
                        if let ::core::option::Option::Some(prepared_cred_protect) =
                            prepared_cred_protect
                        {
                            table_writer.write::<_, _, 1>(3, &prepared_cred_protect);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RegistrationExtensionsClientOutputs>>
                for RegistrationExtensionsClientOutputs
            {
                type Prepared = ::planus::Offset<Self>;

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
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RegistrationExtensionsClientOutputs> {
                    RegistrationExtensionsClientOutputs::create(
                        builder,
                        &self.app_id,
                        &self.cred_props,
                        &self.hmac_secret,
                        &self.cred_protect,
                        &self.min_pin_length,
                    )
                }
            }

            #[derive(Copy, Clone)]
            pub struct RegistrationExtensionsClientOutputsRef<'a>(
                ::planus::table_reader::Table<'a>,
            );

            impl<'a> RegistrationExtensionsClientOutputsRef<'a> {
                pub fn app_id(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "RegistrationExtensionsClientOutputs", "app_id")?
                            .unwrap_or(false),
                    )
                }

                pub fn cred_props(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "RegistrationExtensionsClientOutputs", "cred_props")?
                            .unwrap_or(false),
                    )
                }

                pub fn hmac_secret(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "RegistrationExtensionsClientOutputs", "hmac_secret")?
                            .unwrap_or(false),
                    )
                }

                pub fn cred_protect(&self) -> ::planus::Result<self::CredentialProtectionPolicy> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(3, "RegistrationExtensionsClientOutputs", "cred_protect")?
                            .unwrap_or(self::CredentialProtectionPolicy::UserVerificationOptional),
                    )
                }

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
                pub cred_protect:
                    ::core::option::Option<::planus::alloc::boxed::Box<self::CredProtect>>,
                pub uvm: bool,
                pub cred_props: bool,
                pub min_pin_length: bool,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<12, 8>::new(builder);

                    if prepared_cred_protect.is_some() {
                        table_writer.calculate_size::<::planus::Offset<self::CredProtect>>(2);
                    }
                    if prepared_uvm.is_some() {
                        table_writer.calculate_size::<bool>(4);
                    }
                    if prepared_cred_props.is_some() {
                        table_writer.calculate_size::<bool>(6);
                    }
                    if prepared_min_pin_length.is_some() {
                        table_writer.calculate_size::<bool>(8);
                    }
                    if prepared_hmac_create_secret.is_some() {
                        table_writer.calculate_size::<bool>(10);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_cred_protect) =
                            prepared_cred_protect
                        {
                            table_writer.write::<_, _, 4>(0, &prepared_cred_protect);
                        }
                        if let ::core::option::Option::Some(prepared_uvm) = prepared_uvm {
                            table_writer.write::<_, _, 1>(1, &prepared_uvm);
                        }
                        if let ::core::option::Option::Some(prepared_cred_props) =
                            prepared_cred_props
                        {
                            table_writer.write::<_, _, 1>(2, &prepared_cred_props);
                        }
                        if let ::core::option::Option::Some(prepared_min_pin_length) =
                            prepared_min_pin_length
                        {
                            table_writer.write::<_, _, 1>(3, &prepared_min_pin_length);
                        }
                        if let ::core::option::Option::Some(prepared_hmac_create_secret) =
                            prepared_hmac_create_secret
                        {
                            table_writer.write::<_, _, 1>(4, &prepared_hmac_create_secret);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RequestRegistrationExtensions>>
                for RequestRegistrationExtensions
            {
                type Prepared = ::planus::Offset<Self>;

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

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RequestRegistrationExtensions>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RequestRegistrationExtensions> for RequestRegistrationExtensions {
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RequestRegistrationExtensions> {
                    RequestRegistrationExtensions::create(
                        builder,
                        &self.cred_protect,
                        &self.uvm,
                        &self.cred_props,
                        &self.min_pin_length,
                        &self.hmac_create_secret,
                    )
                }
            }

            #[derive(Copy, Clone)]
            pub struct RequestRegistrationExtensionsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RequestRegistrationExtensionsRef<'a> {
                pub fn cred_protect(
                    &self,
                ) -> ::planus::Result<::core::option::Option<self::CredProtectRef<'a>>>
                {
                    self.0
                        .access(0, "RequestRegistrationExtensions", "cred_protect")
                }

                pub fn uvm(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "RequestRegistrationExtensions", "uvm")?
                            .unwrap_or(false),
                    )
                }

                pub fn cred_props(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(2, "RequestRegistrationExtensions", "cred_props")?
                            .unwrap_or(false),
                    )
                }

                pub fn min_pin_length(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(3, "RequestRegistrationExtensions", "min_pin_length")?
                            .unwrap_or(false),
                    )
                }

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
                pub credential_protection_policy: self::CredentialProtectionPolicy,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<6, 2>::new(builder);

                    if prepared_credential_protection_policy.is_some() {
                        table_writer.calculate_size::<self::CredentialProtectionPolicy>(2);
                    }
                    if prepared_enforce_credential_protection_policy.is_some() {
                        table_writer.calculate_size::<bool>(4);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_credential_protection_policy) =
                            prepared_credential_protection_policy
                        {
                            table_writer
                                .write::<_, _, 1>(0, &prepared_credential_protection_policy);
                        }
                        if let ::core::option::Option::Some(
                            prepared_enforce_credential_protection_policy,
                        ) = prepared_enforce_credential_protection_policy
                        {
                            table_writer.write::<_, _, 1>(
                                1,
                                &prepared_enforce_credential_protection_policy,
                            );
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<CredProtect>> for CredProtect {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<CredProtect>> for CredProtect {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<CredProtect>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<CredProtect> for CredProtect {
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<CredProtect> {
                    CredProtect::create(
                        builder,
                        &self.credential_protection_policy,
                        &self.enforce_credential_protection_policy,
                    )
                }
            }

            #[derive(Copy, Clone)]
            pub struct CredProtectRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> CredProtectRef<'a> {
                pub fn credential_protection_policy(
                    &self,
                ) -> ::planus::Result<self::CredentialProtectionPolicy> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(0, "CredProtect", "credential_protection_policy")?
                            .unwrap_or(self::CredentialProtectionPolicy::UserVerificationOptional),
                    )
                }

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
                UserVerificationOptional = 1,
                UserVerificationOptionalWithCredentialIdList = 2,
                UserVerificationRequired = 3,
            }

            impl ::core::convert::TryFrom<i8> for CredentialProtectionPolicy {
                type Error = ::planus::errors::UnknownEnumTagKind;
                fn try_from(
                    value: i8,
                ) -> ::core::result::Result<Self, ::planus::errors::UnknownEnumTagKind>
                {
                    #[allow(clippy::match_single_binding)]
        match value {
            1 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationOptional),
            2 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationOptionalWithCredentialIdList),
            3 => ::core::result::Result::Ok(CredentialProtectionPolicy::UserVerificationRequired),
            

            _ => ::core::result::Result::Err(::planus::errors::UnknownEnumTagKind { tag: value as i128 }),
        }
                }
            }

            impl ::core::convert::From<CredentialProtectionPolicy> for i8 {
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
                    let value = <i8 as ::planus::VectorRead>::from_buffer(buffer, offset);
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
                pub type_: ::core::option::Option<::planus::alloc::string::String>,
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<8, 12>::new(builder);

                    if prepared_type_.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(2);
                    }
                    if prepared_id.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(4);
                    }
                    if prepared_transports.is_some() {
                        table_writer
                            .calculate_size::<::planus::Offset<[self::AuthenticatorTransport]>>(6);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                            table_writer.write::<_, _, 4>(0, &prepared_type_);
                        }
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            table_writer.write::<_, _, 4>(1, &prepared_id);
                        }
                        if let ::core::option::Option::Some(prepared_transports) =
                            prepared_transports
                        {
                            table_writer.write::<_, _, 4>(2, &prepared_transports);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<PublicKeyCredentialDescriptor>>
                for PublicKeyCredentialDescriptor
            {
                type Prepared = ::planus::Offset<Self>;

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

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PublicKeyCredentialDescriptor>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<PublicKeyCredentialDescriptor> for PublicKeyCredentialDescriptor {
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

            #[derive(Copy, Clone)]
            pub struct PublicKeyCredentialDescriptorRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> PublicKeyCredentialDescriptorRef<'a> {
                pub fn type_(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "PublicKeyCredentialDescriptor", "type_")
                }

                pub fn id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0.access(1, "PublicKeyCredentialDescriptor", "id")
                }

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
                        id: if let ::core::option::Option::Some(id) = value.id()? {
                            ::core::option::Option::Some(id.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
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
                pub authenticator_attachment: self::AuthenticatorAttachment,
                pub require_resident_key: bool,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<8, 3>::new(builder);

                    if prepared_authenticator_attachment.is_some() {
                        table_writer.calculate_size::<self::AuthenticatorAttachment>(2);
                    }
                    if prepared_require_resident_key.is_some() {
                        table_writer.calculate_size::<bool>(4);
                    }
                    if prepared_user_verification.is_some() {
                        table_writer.calculate_size::<self::UserVerificationPolicy>(6);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_authenticator_attachment) =
                            prepared_authenticator_attachment
                        {
                            table_writer.write::<_, _, 1>(0, &prepared_authenticator_attachment);
                        }
                        if let ::core::option::Option::Some(prepared_require_resident_key) =
                            prepared_require_resident_key
                        {
                            table_writer.write::<_, _, 1>(1, &prepared_require_resident_key);
                        }
                        if let ::core::option::Option::Some(prepared_user_verification) =
                            prepared_user_verification
                        {
                            table_writer.write::<_, _, 1>(2, &prepared_user_verification);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<AuthenticatorSelectionCriteria>>
                for AuthenticatorSelectionCriteria
            {
                type Prepared = ::planus::Offset<Self>;

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

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<AuthenticatorSelectionCriteria>>
                {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<AuthenticatorSelectionCriteria> for AuthenticatorSelectionCriteria {
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<AuthenticatorSelectionCriteria> {
                    AuthenticatorSelectionCriteria::create(
                        builder,
                        &self.authenticator_attachment,
                        &self.require_resident_key,
                        &self.user_verification,
                    )
                }
            }

            #[derive(Copy, Clone)]
            pub struct AuthenticatorSelectionCriteriaRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> AuthenticatorSelectionCriteriaRef<'a> {
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

                pub fn require_resident_key(&self) -> ::planus::Result<bool> {
                    ::core::result::Result::Ok(
                        self.0
                            .access(1, "AuthenticatorSelectionCriteria", "require_resident_key")?
                            .unwrap_or(false),
                    )
                }

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
                Preferred = 0,
                Required = 1,
                Discouraged = 2,
            }

            impl ::core::convert::TryFrom<i8> for UserVerificationPolicy {
                type Error = ::planus::errors::UnknownEnumTagKind;
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
                    let value = <i8 as ::planus::VectorRead>::from_buffer(buffer, offset);
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
                Platform = 0,
                CrossPlatform = 1,
            }

            impl ::core::convert::TryFrom<i8> for AuthenticatorAttachment {
                type Error = ::planus::errors::UnknownEnumTagKind;
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
                    let value = <i8 as ::planus::VectorRead>::from_buffer(buffer, offset);
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
                Usb = 0,
                Nfc = 1,
                Ble = 2,
                Internal = 3,
                Hybrid = 4,
                Test = 5,
            }

            impl ::core::convert::TryFrom<i8> for AuthenticatorTransport {
                type Error = ::planus::errors::UnknownEnumTagKind;
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
                    let value = <i8 as ::planus::VectorRead>::from_buffer(buffer, offset);
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
                None = 0,
                Indirect = 1,
                Direct = 2,
            }

            impl ::core::convert::TryFrom<i8> for AttestationConveyancePreference {
                type Error = ::planus::errors::UnknownEnumTagKind;
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
                    let value = <i8 as ::planus::VectorRead>::from_buffer(buffer, offset);
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
                pub type_: ::core::option::Option<::planus::alloc::string::String>,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<6, 8>::new(builder);

                    if prepared_type_.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(2);
                    }
                    if prepared_alg.is_some() {
                        table_writer.calculate_size::<i32>(4);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_type_) = prepared_type_ {
                            table_writer.write::<_, _, 4>(0, &prepared_type_);
                        }
                        if let ::core::option::Option::Some(prepared_alg) = prepared_alg {
                            table_writer.write::<_, _, 4>(1, &prepared_alg);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<PubKeyCredParams>> for PubKeyCredParams {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<PubKeyCredParams>> for PubKeyCredParams {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<PubKeyCredParams>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<PubKeyCredParams> for PubKeyCredParams {
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<PubKeyCredParams> {
                    PubKeyCredParams::create(builder, &self.type_, &self.alg)
                }
            }

            #[derive(Copy, Clone)]
            pub struct PubKeyCredParamsRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> PubKeyCredParamsRef<'a> {
                pub fn type_(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "PubKeyCredParams", "type_")
                }

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
                pub id: ::core::option::Option<::planus::alloc::string::String>,
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
                #[allow(clippy::too_many_arguments)]
                pub fn create(
                    builder: &mut ::planus::Builder,
                    field_id: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                    field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                ) -> ::planus::Offset<Self> {
                    let prepared_id = field_id.prepare(builder);

                    let prepared_name = field_name.prepare(builder);

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<6, 8>::new(builder);

                    if prepared_id.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(2);
                    }
                    if prepared_name.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(4);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            table_writer.write::<_, _, 4>(0, &prepared_id);
                        }
                        if let ::core::option::Option::Some(prepared_name) = prepared_name {
                            table_writer.write::<_, _, 4>(1, &prepared_name);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<RelyingParty>> for RelyingParty {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<RelyingParty>> for RelyingParty {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<RelyingParty>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<RelyingParty> for RelyingParty {
                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::planus::Offset<RelyingParty> {
                    RelyingParty::create(builder, &self.id, &self.name)
                }
            }

            #[derive(Copy, Clone)]
            pub struct RelyingPartyRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> RelyingPartyRef<'a> {
                pub fn id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(0, "RelyingParty", "id")
                }

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
                pub id: ::core::option::Option<::planus::alloc::vec::Vec<u8>>,
                pub name: ::core::option::Option<::planus::alloc::string::String>,
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

                    let mut table_writer =
                        ::planus::table_writer::TableWriter::<8, 12>::new(builder);

                    if prepared_id.is_some() {
                        table_writer.calculate_size::<::planus::Offset<[u8]>>(2);
                    }
                    if prepared_name.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(4);
                    }
                    if prepared_display_name.is_some() {
                        table_writer.calculate_size::<::planus::Offset<str>>(6);
                    }

                    table_writer.finish_calculating();

                    unsafe {
                        if let ::core::option::Option::Some(prepared_id) = prepared_id {
                            table_writer.write::<_, _, 4>(0, &prepared_id);
                        }
                        if let ::core::option::Option::Some(prepared_name) = prepared_name {
                            table_writer.write::<_, _, 4>(1, &prepared_name);
                        }
                        if let ::core::option::Option::Some(prepared_display_name) =
                            prepared_display_name
                        {
                            table_writer.write::<_, _, 4>(2, &prepared_display_name);
                        }
                    }

                    table_writer.finish()
                }
            }

            impl ::planus::WriteAs<::planus::Offset<User>> for User {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    ::planus::WriteAsOffset::prepare(self, builder)
                }
            }

            impl ::planus::WriteAsOptional<::planus::Offset<User>> for User {
                type Prepared = ::planus::Offset<Self>;

                fn prepare(
                    &self,
                    builder: &mut ::planus::Builder,
                ) -> ::core::option::Option<::planus::Offset<User>> {
                    ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
                }
            }

            impl ::planus::WriteAsOffset<User> for User {
                fn prepare(&self, builder: &mut ::planus::Builder) -> ::planus::Offset<User> {
                    User::create(builder, &self.id, &self.name, &self.display_name)
                }
            }

            #[derive(Copy, Clone)]
            pub struct UserRef<'a>(::planus::table_reader::Table<'a>);

            impl<'a> UserRef<'a> {
                pub fn id(
                    &self,
                ) -> ::planus::Result<::core::option::Option<::planus::Vector<'a, u8>>>
                {
                    self.0.access(0, "User", "id")
                }

                pub fn name(
                    &self,
                ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>>
                {
                    self.0.access(1, "User", "name")
                }

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
                        id: if let ::core::option::Option::Some(id) = value.id()? {
                            ::core::option::Option::Some(id.to_vec()?)
                        } else {
                            ::core::option::Option::None
                        },
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
            pub name: ::core::option::Option<::planus::alloc::string::String>,
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
            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_name: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_number: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            ) -> ::planus::Offset<Self> {
                let prepared_name = field_name.prepare(builder);

                let prepared_number = field_number.prepare(builder);

                let mut table_writer = ::planus::table_writer::TableWriter::<6, 8>::new(builder);

                if prepared_name.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(2);
                }
                if prepared_number.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(4);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_name) = prepared_name {
                        table_writer.write::<_, _, 4>(0, &prepared_name);
                    }
                    if let ::core::option::Option::Some(prepared_number) = prepared_number {
                        table_writer.write::<_, _, 4>(1, &prepared_number);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<RegisterNumberReq>> for RegisterNumberReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<RegisterNumberReq>> for RegisterNumberReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<RegisterNumberReq> for RegisterNumberReq {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberReq> {
                RegisterNumberReq::create(builder, &self.name, &self.number)
            }
        }

        #[derive(Copy, Clone)]
        pub struct RegisterNumberReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> RegisterNumberReqRef<'a> {
            pub fn name(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "RegisterNumberReq", "name")
            }

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
            pub success: bool,
            pub multiplier: u16,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for RegisterNumberRes {
            fn default() -> Self {
                Self {
                    success: true,
                    multiplier: 0,
                }
            }
        }

        impl RegisterNumberRes {
            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_success: impl ::planus::WriteAsDefault<bool, bool>,
                field_multiplier: impl ::planus::WriteAsDefault<u16, u16>,
            ) -> ::planus::Offset<Self> {
                let prepared_success = field_success.prepare(builder, &true);

                let prepared_multiplier = field_multiplier.prepare(builder, &0);

                let mut table_writer = ::planus::table_writer::TableWriter::<6, 3>::new(builder);

                if prepared_success.is_some() {
                    table_writer.calculate_size::<bool>(2);
                }
                if prepared_multiplier.is_some() {
                    table_writer.calculate_size::<u16>(4);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_multiplier) = prepared_multiplier {
                        table_writer.write::<_, _, 2>(1, &prepared_multiplier);
                    }
                    if let ::core::option::Option::Some(prepared_success) = prepared_success {
                        table_writer.write::<_, _, 1>(0, &prepared_success);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<RegisterNumberRes>> for RegisterNumberRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<RegisterNumberRes>> for RegisterNumberRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<RegisterNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<RegisterNumberRes> for RegisterNumberRes {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<RegisterNumberRes> {
                RegisterNumberRes::create(builder, &self.success, &self.multiplier)
            }
        }

        #[derive(Copy, Clone)]
        pub struct RegisterNumberResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> RegisterNumberResRef<'a> {
            pub fn success(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(
                    self.0
                        .access(0, "RegisterNumberRes", "success")?
                        .unwrap_or(true),
                )
            }

            pub fn multiplier(&self) -> ::planus::Result<u16> {
                ::core::result::Result::Ok(
                    self.0
                        .access(1, "RegisterNumberRes", "multiplier")?
                        .unwrap_or(0),
                )
            }
        }

        impl<'a> ::core::fmt::Debug for RegisterNumberResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("RegisterNumberResRef");
                f.field("success", &self.success());
                f.field("multiplier", &self.multiplier());
                f.finish()
            }
        }

        impl<'a> ::core::convert::TryFrom<RegisterNumberResRef<'a>> for RegisterNumberRes {
            type Error = ::planus::Error;

            #[allow(unreachable_code)]
            fn try_from(value: RegisterNumberResRef<'a>) -> ::planus::Result<Self> {
                ::core::result::Result::Ok(Self {
                    success: ::core::convert::TryInto::try_into(value.success()?)?,
                    multiplier: ::core::convert::TryInto::try_into(value.multiplier()?)?,
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for RegisterNumberResRef<'a> {
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
            pub number: ::core::option::Option<::planus::alloc::string::String>,
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
            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_number: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
                field_code: impl ::planus::WriteAsDefault<u16, u16>,
            ) -> ::planus::Offset<Self> {
                let prepared_number = field_number.prepare(builder);

                let prepared_code = field_code.prepare(builder, &0);

                let mut table_writer = ::planus::table_writer::TableWriter::<6, 6>::new(builder);

                if prepared_number.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(2);
                }
                if prepared_code.is_some() {
                    table_writer.calculate_size::<u16>(4);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_number) = prepared_number {
                        table_writer.write::<_, _, 4>(0, &prepared_number);
                    }
                    if let ::core::option::Option::Some(prepared_code) = prepared_code {
                        table_writer.write::<_, _, 2>(1, &prepared_code);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyNumberReq>> for VerifyNumberReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyNumberReq>> for VerifyNumberReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyNumberReq> for VerifyNumberReq {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberReq> {
                VerifyNumberReq::create(builder, &self.number, &self.code)
            }
        }

        #[derive(Copy, Clone)]
        pub struct VerifyNumberReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyNumberReqRef<'a> {
            pub fn number(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "VerifyNumberReq", "number")
            }

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
            pub success: bool,
            pub options: ::core::option::Option<
                ::planus::alloc::boxed::Box<self::web_authn::CredentialCreationOptions>,
            >,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyNumberRes {
            fn default() -> Self {
                Self {
                    success: true,
                    options: ::core::default::Default::default(),
                }
            }
        }

        impl VerifyNumberRes {
            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_success: impl ::planus::WriteAsDefault<bool, bool>,
                field_options: impl ::planus::WriteAsOptional<
                    ::planus::Offset<self::web_authn::CredentialCreationOptions>,
                >,
            ) -> ::planus::Offset<Self> {
                let prepared_success = field_success.prepare(builder, &true);

                let prepared_options = field_options.prepare(builder);

                let mut table_writer = ::planus::table_writer::TableWriter::<6, 5>::new(builder);

                if prepared_success.is_some() {
                    table_writer.calculate_size::<bool>(2);
                }
                if prepared_options.is_some() {
                    table_writer.calculate_size::<::planus::Offset<self::web_authn::CredentialCreationOptions>>(4);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_options) = prepared_options {
                        table_writer.write::<_, _, 4>(1, &prepared_options);
                    }
                    if let ::core::option::Option::Some(prepared_success) = prepared_success {
                        table_writer.write::<_, _, 1>(0, &prepared_success);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyNumberRes>> for VerifyNumberRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyNumberRes>> for VerifyNumberRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyNumberRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyNumberRes> for VerifyNumberRes {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyNumberRes> {
                VerifyNumberRes::create(builder, &self.success, &self.options)
            }
        }

        #[derive(Copy, Clone)]
        pub struct VerifyNumberResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyNumberResRef<'a> {
            pub fn success(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(
                    self.0
                        .access(0, "VerifyNumberRes", "success")?
                        .unwrap_or(true),
                )
            }

            pub fn options(
                &self,
            ) -> ::planus::Result<
                ::core::option::Option<self::web_authn::CredentialCreationOptionsRef<'a>>,
            > {
                self.0.access(1, "VerifyNumberRes", "options")
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyNumberResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyNumberResRef");
                f.field("success", &self.success());
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
                    success: ::core::convert::TryInto::try_into(value.success()?)?,
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
            pub name: ::core::option::Option<::planus::alloc::string::String>,
            pub id: ::core::option::Option<::planus::alloc::string::String>,
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

                let mut table_writer = ::planus::table_writer::TableWriter::<8, 12>::new(builder);

                if prepared_name.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(2);
                }
                if prepared_id.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(4);
                }
                if prepared_webauthn.is_some() {
                    table_writer.calculate_size::<::planus::Offset<self::web_authn::RegisterPublicKeyCredential>>(6);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_name) = prepared_name {
                        table_writer.write::<_, _, 4>(0, &prepared_name);
                    }
                    if let ::core::option::Option::Some(prepared_id) = prepared_id {
                        table_writer.write::<_, _, 4>(1, &prepared_id);
                    }
                    if let ::core::option::Option::Some(prepared_webauthn) = prepared_webauthn {
                        table_writer.write::<_, _, 4>(2, &prepared_webauthn);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyWebAuthnReq>> for VerifyWebAuthnReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnReq>> for VerifyWebAuthnReq {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnReq>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyWebAuthnReq> for VerifyWebAuthnReq {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnReq> {
                VerifyWebAuthnReq::create(builder, &self.name, &self.id, &self.webauthn)
            }
        }

        #[derive(Copy, Clone)]
        pub struct VerifyWebAuthnReqRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyWebAuthnReqRef<'a> {
            pub fn name(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(0, "VerifyWebAuthnReq", "name")
            }

            pub fn id(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(1, "VerifyWebAuthnReq", "id")
            }

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
            pub success: bool,
            pub token: ::core::option::Option<::planus::alloc::string::String>,
        }

        #[allow(clippy::derivable_impls)]
        impl ::core::default::Default for VerifyWebAuthnRes {
            fn default() -> Self {
                Self {
                    success: true,
                    token: ::core::default::Default::default(),
                }
            }
        }

        impl VerifyWebAuthnRes {
            #[allow(clippy::too_many_arguments)]
            pub fn create(
                builder: &mut ::planus::Builder,
                field_success: impl ::planus::WriteAsDefault<bool, bool>,
                field_token: impl ::planus::WriteAsOptional<::planus::Offset<::core::primitive::str>>,
            ) -> ::planus::Offset<Self> {
                let prepared_success = field_success.prepare(builder, &true);

                let prepared_token = field_token.prepare(builder);

                let mut table_writer = ::planus::table_writer::TableWriter::<6, 5>::new(builder);

                if prepared_success.is_some() {
                    table_writer.calculate_size::<bool>(2);
                }
                if prepared_token.is_some() {
                    table_writer.calculate_size::<::planus::Offset<str>>(4);
                }

                table_writer.finish_calculating();

                unsafe {
                    if let ::core::option::Option::Some(prepared_token) = prepared_token {
                        table_writer.write::<_, _, 4>(1, &prepared_token);
                    }
                    if let ::core::option::Option::Some(prepared_success) = prepared_success {
                        table_writer.write::<_, _, 1>(0, &prepared_success);
                    }
                }

                table_writer.finish()
            }
        }

        impl ::planus::WriteAs<::planus::Offset<VerifyWebAuthnRes>> for VerifyWebAuthnRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                ::planus::WriteAsOffset::prepare(self, builder)
            }
        }

        impl ::planus::WriteAsOptional<::planus::Offset<VerifyWebAuthnRes>> for VerifyWebAuthnRes {
            type Prepared = ::planus::Offset<Self>;

            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::core::option::Option<::planus::Offset<VerifyWebAuthnRes>> {
                ::core::option::Option::Some(::planus::WriteAsOffset::prepare(self, builder))
            }
        }

        impl ::planus::WriteAsOffset<VerifyWebAuthnRes> for VerifyWebAuthnRes {
            fn prepare(
                &self,
                builder: &mut ::planus::Builder,
            ) -> ::planus::Offset<VerifyWebAuthnRes> {
                VerifyWebAuthnRes::create(builder, &self.success, &self.token)
            }
        }

        #[derive(Copy, Clone)]
        pub struct VerifyWebAuthnResRef<'a>(::planus::table_reader::Table<'a>);

        impl<'a> VerifyWebAuthnResRef<'a> {
            pub fn success(&self) -> ::planus::Result<bool> {
                ::core::result::Result::Ok(
                    self.0
                        .access(0, "VerifyWebAuthnRes", "success")?
                        .unwrap_or(true),
                )
            }

            pub fn token(
                &self,
            ) -> ::planus::Result<::core::option::Option<&'a ::core::primitive::str>> {
                self.0.access(1, "VerifyWebAuthnRes", "token")
            }
        }

        impl<'a> ::core::fmt::Debug for VerifyWebAuthnResRef<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut f = f.debug_struct("VerifyWebAuthnResRef");
                f.field("success", &self.success());
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
                    success: ::core::convert::TryInto::try_into(value.success()?)?,
                    token: if let ::core::option::Option::Some(token) = value.token()? {
                        ::core::option::Option::Some(::core::convert::TryInto::try_into(token)?)
                    } else {
                        ::core::option::Option::None
                    },
                })
            }
        }

        impl<'a> ::planus::TableRead<'a> for VerifyWebAuthnResRef<'a> {
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
    }
}
