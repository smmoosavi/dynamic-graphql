mod resolved_object_lifetime_tests {
    use dynamic_graphql::dynamic::{DynamicRequestExt, ResolverContext};
    use dynamic_graphql::App;
    use dynamic_graphql::FieldValue;
    use dynamic_graphql::ResolvedObject;
    use dynamic_graphql::ResolvedObjectFields;
    struct Outer {
        string: String,
    }
    struct Inner<'a> {
        pub string_ref: &'a str,
    }
    impl Outer {
        fn get_inner(&self) -> Inner<'_> {
            Inner {
                string_ref: &self.string,
            }
        }
    }
    struct Query {
        pub outer: Outer,
    }
    impl Query {
        fn __registers(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            registry
        }
    }
    impl dynamic_graphql::internal::ParentType for Query {
        type Type = Query;
    }
    impl dynamic_graphql::internal::TypeName for Query {
        fn get_type_name() -> std::borrow::Cow<'static, str> {
            "Query".into()
        }
    }
    impl dynamic_graphql::internal::OutputTypeName for Query {}
    impl dynamic_graphql::internal::Object for Query {}
    impl<'__dynamic_graphql_lifetime>
        dynamic_graphql::internal::ResolveOwned<'__dynamic_graphql_lifetime> for Query
    {
        fn resolve_owned(
            self,
            _ctx: &dynamic_graphql::Context,
        ) -> dynamic_graphql::Result<Option<dynamic_graphql::FieldValue<'__dynamic_graphql_lifetime>>>
        {
            Ok(Some(dynamic_graphql::FieldValue::owned_any(self)))
        }
    }
    impl<'__dynamic_graphql_lifetime>
        dynamic_graphql::internal::ResolveRef<'__dynamic_graphql_lifetime> for Query
    {
        fn resolve_ref(
            &'__dynamic_graphql_lifetime self,
            _ctx: &dynamic_graphql::Context,
        ) -> dynamic_graphql::Result<Option<dynamic_graphql::FieldValue<'__dynamic_graphql_lifetime>>>
        {
            Ok(Some(dynamic_graphql::FieldValue::borrowed_any(self)))
        }
    }
    impl Query {
        fn __register_doc(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            registry
        }
    }
    impl Query {
        fn __register_interface(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            let registry = registry.update_object(
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
                |object| object,
            );
            registry
        }
    }
    impl Query {
        fn __register_root(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            let registry = registry.set_root(
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
            );
            registry
        }
    }
    impl dynamic_graphql::internal::RegisterFns for Query {
        const REGISTER_FNS: &'static [fn(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry] = &[
            Query::__register_interface,
            Query::__register_root,
            Query::__register_doc,
            Query::__registers,
        ];
    }
    struct Foo<'a> {
        pub inner: Inner<'a>,
    }
    impl<'a> Foo<'a> {
        fn __registers(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            registry
        }
    }
    impl<'a: 'static> dynamic_graphql::internal::ParentType for Foo<'a> {
        type Type = Foo<'a>;
    }
    impl<'a: 'static> dynamic_graphql::internal::TypeName for Foo<'a> {
        fn get_type_name() -> std::borrow::Cow<'static, str> {
            "Foo".into()
        }
    }
    impl<'a: 'static> dynamic_graphql::internal::OutputTypeName for Foo<'a> {}
    impl<'a: 'static> dynamic_graphql::internal::Object for Foo<'a> {}
    impl<'a: 'static, '__dynamic_graphql_lifetime>
        dynamic_graphql::internal::ResolveOwned<'__dynamic_graphql_lifetime> for Foo<'a>
    {
        fn resolve_owned(
            self,
            _ctx: &dynamic_graphql::Context,
        ) -> dynamic_graphql::Result<Option<dynamic_graphql::FieldValue<'__dynamic_graphql_lifetime>>>
        {
            Ok(Some(dynamic_graphql::FieldValue::owned_any(self)))
        }
    }
    impl<'a: 'static, '__dynamic_graphql_lifetime>
        dynamic_graphql::internal::ResolveRef<'__dynamic_graphql_lifetime> for Foo<'a>
    {
        fn resolve_ref(
            &'__dynamic_graphql_lifetime self,
            _ctx: &dynamic_graphql::Context,
        ) -> dynamic_graphql::Result<Option<dynamic_graphql::FieldValue<'__dynamic_graphql_lifetime>>>
        {
            Ok(Some(dynamic_graphql::FieldValue::borrowed_any(self)))
        }
    }
    impl<'a> Foo<'a> {
        fn __register_doc(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            registry
        }
    }
    impl<'a: 'static> Foo<'a> {
        fn __register_interface(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            let registry = registry.update_object(
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
                |object| object,
            );
            registry
        }
    }
    impl<'a> Foo<'a> {
        fn __register_root(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            registry
        }
    }
    impl<'a: 'static> dynamic_graphql::internal::RegisterFns for Foo<'a> {
        const REGISTER_FNS: &'static [fn(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry] = &[
            Foo::<'a>::__register_interface,
            Foo::<'a>::__register_root,
            Foo::<'a>::__register_doc,
            Foo::<'a>::__registers,
        ];
    }
    impl Query {
        fn string(&self) -> String {
            self.outer.string.clone()
        }
        fn string_ref(&self) -> &str {
            &self.outer.string
        }
        fn foo<'a>(&'a self) -> Foo<'a> {
            Foo {
                inner: Inner {
                    string_ref: &self.outer.string,
                },
            }
        }
    }
    impl dynamic_graphql::internal::Register for Query {
        fn register(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            let registry = registry.register::<&str>();
            let registry = registry.register::<Foo>();
            let registry = registry.register::<String>();
            let object = dynamic_graphql::dynamic::Object::new(
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
            );
            let field = dynamic_graphql::dynamic::Field::new(
                "string",
                <String as dynamic_graphql::internal::GetOutputTypeRef>::get_output_type_ref(),
                |ctx| {
                    dynamic_graphql::dynamic::FieldFuture::new(async move {
                        let parent = ctx
                            .parent_value
                            .try_downcast_ref::<
                                <Self as dynamic_graphql::internal::ParentType>::Type,
                            >()?
                            .into();
                        let arg0 = parent;
                        let value = Self::string(arg0);
                        dynamic_graphql::internal::Resolve::resolve(value, &ctx)
                    })
                },
            );
            let object = object.field(field);
            let field = dynamic_graphql::dynamic::Field::new(
                "stringRef",
                <&str as dynamic_graphql::internal::GetOutputTypeRef>::get_output_type_ref(),
                |ctx| {
                    dynamic_graphql::dynamic::FieldFuture::new(async move {
                        let parent = ctx
                            .parent_value
                            .try_downcast_ref::<
                                <Self as dynamic_graphql::internal::ParentType>::Type,
                            >()?
                            .into();
                        let arg0 = parent;
                        let value = Self::string_ref(arg0);
                        dynamic_graphql::internal::Resolve::resolve(value, &ctx)
                    })
                },
            );
            let object = object.field(field);
            let field = dynamic_graphql::dynamic::Field::new(
                "foo",
                <Foo as dynamic_graphql::internal::GetOutputTypeRef>::get_output_type_ref(),
                |ctx:ResolverContext<'_>| {
                    dynamic_graphql::dynamic::FieldFuture::new(async move {
                        let parent = ctx
                            .parent_value
                            .try_downcast_ref::<
                                Query,
                            >()?
                            .into();
                        let arg0 = parent;
                        let value = Self::foo(arg0);
                        dynamic_graphql::internal::Resolve::resolve(value, &ctx)
                    })
                },
            );
            let object = object.field(field);
            let registry = <Self as dynamic_graphql::internal::RegisterFns>::REGISTER_FNS
                .iter()
                .fold(registry, |registry, f| f(registry));
            registry.register_type(object)
        }
    }
    impl<'a> Foo<'a> {
        fn string(&self) -> String {
            self.inner.string_ref.to_string()
        }
        fn string_ref(&self) -> &str {
            self.inner.string_ref
        }
    }
    impl<'a: 'static> dynamic_graphql::internal::Register for Foo<'a> {
        fn register(
            registry: dynamic_graphql::internal::Registry,
        ) -> dynamic_graphql::internal::Registry {
            let registry = registry.register::<&str>();
            let registry = registry.register::<String>();
            let object = dynamic_graphql::dynamic::Object::new(
                <Self as dynamic_graphql::internal::Object>::get_object_type_name().as_ref(),
            );
            let field = dynamic_graphql::dynamic::Field::new(
                "string",
                <String as dynamic_graphql::internal::GetOutputTypeRef>::get_output_type_ref(),
                |ctx| {
                    dynamic_graphql::dynamic::FieldFuture::new(async move {
                        let parent = ctx
                            .parent_value
                            .try_downcast_ref::<
                                <Self as dynamic_graphql::internal::ParentType>::Type,
                            >()?
                            .into();
                        let arg0 = parent;
                        let value = Self::string(arg0);
                        dynamic_graphql::internal::Resolve::resolve(value, &ctx)
                    })
                },
            );
            let object = object.field(field);
            let field = dynamic_graphql::dynamic::Field::new(
                "stringRef",
                <&str as dynamic_graphql::internal::GetOutputTypeRef>::get_output_type_ref(),
                |ctx| {
                    dynamic_graphql::dynamic::FieldFuture::new(async move {
                        let parent = ctx
                            .parent_value
                            .try_downcast_ref::<
                                <Self as dynamic_graphql::internal::ParentType>::Type,
                            >()?
                            .into();
                        let arg0 = parent;
                        let value = Self::string_ref(arg0);
                        dynamic_graphql::internal::Resolve::resolve(value, &ctx)
                    })
                },
            );
            let object = object.field(field);
            let registry = <Self as dynamic_graphql::internal::RegisterFns>::REGISTER_FNS
                .iter()
                .fold(registry, |registry, f| f(registry));
            registry.register_type(object)
        }
    }
}
