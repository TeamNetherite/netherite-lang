use ry_ast::{location::WithSpan, *};
use std::ops::Deref;

pub struct GraphvizTranslatorState {
    current_node_index: u32,
}

impl Default for GraphvizTranslatorState {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphvizTranslatorState {
    pub fn new() -> Self {
        Self {
            current_node_index: 0,
        }
    }

    pub fn ast_to_graphviz(&mut self, ast: &ProgramUnit) {
        println!("digraph {{");
        for import in &ast.imports {
            self.create_import_node(&import);
        }
        for stmt in &ast.top_level_statements {
            self.create_top_level_stmt_node(&stmt.0);
        }
        println!("}}");
    }

    fn create_import_node(&mut self, import: &Import) -> u32 {
        let import_node = self.add_node("Import");
        let filename_node = self.add_node(&import.filename.value);
        self.add_node_connections(&[import_node, filename_node]);
        import_node
    }

    fn create_top_level_stmt_node(&mut self, stmt: &TopLevelStatement) -> u32 {
        match stmt {
            TopLevelStatement::FunctionDecl(ref f) => {
                let root = self.add_node("FunDecl");
                let name_node_root = self.add_node("Name");
                let name_node = self.add_node(&f.def.name.value);

                self.add_node_connections(&[root, name_node_root, name_node]);

                if f.def.public.is_some() {
                    let public_node = self.add_node("Public");
                    self.add_node_connections(&[root, public_node]);
                }

                if !f.def.generic_annotations.is_empty() {
                    let generics_node =
                        self.create_generic_annotations_node(&f.def.generic_annotations);
                    self.add_node_connections(&[root, generics_node]);
                }

                if !f.def.params.is_empty() {
                    let params_node = self.create_params_node(&f.def.params);
                    self.add_node_connections(&[root, params_node]);
                }

                let statements_block_node = self.create_statements_block_node(&f.stmts);
                self.add_node_connections(&[root, statements_block_node]);

                if let Some(t) = &f.def.return_type {
                    let return_type_node_root = self.add_node("ReturnType");
                    let return_type_node = self.create_type_node(t.value.deref());
                    self.add_node_connections(&[root, return_type_node_root, return_type_node]);
                }

                root
            }
            TopLevelStatement::StructDecl(ref sd) => {
                let root = self.add_node("StructDecl");

                let name_node_root = self.add_node("Name");
                let name_node = self.add_node(&sd.name.value);

                self.add_node_connections(&[root, name_node_root, name_node]);

                if sd.public.is_some() {
                    let public_node = self.add_node("Public");
                    self.add_node_connections(&[root, public_node]);
                }

                if !sd.generic_annotations.is_empty() {
                    let generics_node =
                        self.create_generic_annotations_node(&sd.generic_annotations);
                    self.add_node_connections(&[root, generics_node]);
                }

                if !sd.members.is_empty() {
                    let members_node = self.add_node("Members");

                    for member in &sd.members {
                        let member_node = self.add_node("Member");

                        self.add_node_connections(&[members_node, member_node]);

                        if member.public.is_some() {
                            let public_node = self.add_node("Public");
                            self.add_node_connections(&[member_node, public_node]);
                        }

                        let name_node_root = self.add_node("Name");
                        let name_node = self.add_node(&member.name.value);

                        self.add_node_connections(&[member_node, name_node_root, name_node]);

                        let type_node_root = self.add_node("Type");
                        let type_node = self.create_type_node(member.ty.value.deref());

                        self.add_node_connections(&[member_node, type_node_root, type_node]);
                    }

                    self.add_node_connections(&[root, members_node]);
                }

                root
            }
            TopLevelStatement::InterfaceDecl(ref i) => {
                let root = self.add_node("InterfaceDecl");

                let name_node_root = self.add_node("Name");
                let name_node = self.add_node(&i.name.value);

                self.add_node_connections(&[root, name_node_root, name_node]);

                if i.public.is_some() {
                    let public_node = self.add_node("Public");
                    self.add_node_connections(&[root, public_node]);
                }

                if !i.generic_annotations.is_empty() {
                    let generics_node =
                        self.create_generic_annotations_node(&i.generic_annotations);
                    self.add_node_connections(&[root, generics_node]);
                }

                if !i.methods.is_empty() {
                    let methods_node = self.add_node("Methods");

                    for method in &i.methods {
                        let method_node = self.add_node("Method");

                        let name_node_root = self.add_node("Name");
                        let name_node = self.add_node(&method.name.value);

                        self.add_node_connections(&[
                            methods_node,
                            method_node,
                            name_node_root,
                            name_node,
                        ]);

                        if !method.generic_annotations.is_empty() {
                            let generics_node =
                                self.create_generic_annotations_node(&method.generic_annotations);

                            self.add_node_connections(&[method_node, generics_node]);
                        }

                        if !method.params.is_empty() {
                            let params_node = self.create_params_node(&method.params);
                            self.add_node_connections(&[method_node, params_node]);
                        }

                        if method.return_type.is_some() {
                            let return_type_node_root = self.add_node("ReturnType");
                            let return_type_node = self.create_type_node(
                                method.return_type.as_ref().unwrap().value.deref(),
                            );

                            self.add_node_connections(&[
                                method_node,
                                return_type_node_root,
                                return_type_node,
                            ]);
                        }
                    }

                    self.add_node_connections(&[root, methods_node]);
                }

                root
            }
            _ => unimplemented!(),
        }
    }

    fn create_statements_block_node(&mut self, statements: &Vec<Statement>) -> u32 {
        let root = self.add_node("StatementsBlock");

        for statement in statements {
            let statement_node_root = self.add_node("Statement");
            let statement_node = self.create_statement_node(statement);

            self.add_node_connections(&[root, statement_node_root, statement_node]);
        }

        root
    }

    fn create_statement_node(&mut self, statement: &Statement) -> u32 {
        match statement {
            Statement::Return(e) => {
                let node = self.add_node("ReturnStatement");
                let expr_node_root = self.add_node("Expression");
                let expr_node = self.create_expression_node(e.value.deref());

                self.add_node_connections(&[node, expr_node_root, expr_node]);

                node
            }
            Statement::ExpressionWithoutSemicolon(e) => {
                let node = self.add_node("ExpressionStatementWithoutSemicolon");
                let expr_node = self.create_expression_node(e.value.deref());

                self.add_node_connections(&[node, expr_node]);

                node
            }
            Statement::Defer(d) => {
                let node = self.add_node("DeferStatement");
                let expr_node_root = self.add_node("Expression");
                let expr_node = self.create_expression_node(d.value.deref());

                self.add_node_connections(&[node, expr_node_root, expr_node]);

                node
            }
            Statement::Expression(e) => {
                let node = self.add_node("ExpressionStatement");
                let expr_node_root = self.add_node("Expression");
                let expr_node = self.create_expression_node(e.value.deref());

                self.add_node_connections(&[node, expr_node_root, expr_node]);

                node
            }
        }
    }

    fn create_params_node(&mut self, params: &Vec<FunctionParam>) -> u32 {
        let params_node = self.add_node("Params");

        for param in params {
            let param_node = self.add_node("Param");
            let param_name_node_root = self.add_node("Name");
            let param_name_node = self.add_node(&param.name.value);

            self.add_node_connections(&[
                params_node,
                param_node,
                param_name_node_root,
                param_name_node,
            ]);

            if let Some(value) = &param.default_value {
                let value_node = self.add_node("Default value");
                let expr_node = self.create_expression_node(value.value.deref());

                self.add_node_connections(&[param_node, value_node, expr_node]);
            }

            let type_node_root = self.add_node("Type");
            let type_node = self.create_type_node(param.ty.value.deref());

            self.add_node_connections(&[param_node, type_node_root, type_node]);
        }

        params_node
    }

    fn create_generic_annotations_node(
        &mut self,
        annotations: &Vec<(WithSpan<String>, Option<Type>)>,
    ) -> u32 {
        let generics_node = self.add_node("Generics");

        for generic in annotations {
            let generic_node_root = self.add_node("Generic");
            let generic_node = self.add_node(&generic.0.value);
            self.add_node_connections(&[generics_node, generic_node_root, generic_node]);
        }

        generics_node
    }

    fn create_expression_node(&mut self, expression: &RawExpression) -> u32 {
        match expression {
            RawExpression::Int(i) => {
                let root = self.add_node("Int");
                let node = self.add_node(&i.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::Float(f) => {
                let root = self.add_node("Float");
                let node = self.add_node(&f.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::Imag(f) => {
                let root = self.add_node("Imag");
                let node = self.add_node(&f.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::String(str) => {
                let root = self.add_node("String");
                let node = self.add_node(&str.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::Bool(b) => {
                let root = self.add_node("Bool");
                let node = self.add_node(&b.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::List(l) => {
                let root = self.add_node("ListExpr");

                for expr in l {
                    let elem = self.add_node("Elem");
                    let expr_node = self.create_expression_node(expr.value.deref());

                    self.add_node_connections(&[root, elem, expr_node]);
                }

                root
            }
            RawExpression::StaticName(name) => {
                let root = self.add_node("StaticName");
                let node = self.add_node(name);

                self.add_node_connections(&[root, node]);

                root
            }
            RawExpression::Binary(lhs, op, rhs) => {
                let root = self.add_node("BinaryExpr");
                let op_node_root = self.add_node("Op");
                let op_node = self.add_node(&op.value.to_string());

                self.add_node_connections(&[root, op_node_root, op_node]);

                let lhs_node_root = self.add_node("LHS");
                let lhs_node = self.create_expression_node(lhs.value.deref());

                let rhs_node_root = self.add_node("RHS");
                let rhs_node = self.create_expression_node(rhs.value.deref());

                self.add_node_connections(&[root, lhs_node_root, lhs_node]);
                self.add_node_connections(&[root, rhs_node_root, rhs_node]);

                root
            }
            RawExpression::Call(generics, caller, params) => {
                let root = self.add_node("Call");

                let caller_node_root = self.add_node("Caller");
                let caller_node = self.create_expression_node(caller.value.deref());

                self.add_node_connections(&[root, caller_node_root, caller_node]);

                if !params.is_empty() {
                    let params_node_root = self.add_node("Params");

                    for param in params {
                        let param_node_root = self.add_node("Param");
                        let param_node = self.create_expression_node(param.value.deref());

                        self.add_node_connections(&[params_node_root, param_node_root, param_node]);
                    }

                    self.add_node_connections(&[root, params_node_root]);
                }

                if !generics.is_empty() {
                    let generics_node_root = self.add_node("Generics");

                    for generic in generics {
                        let generic_node_root = self.add_node("Generic");
                        let generic_node = self.create_type_node(generic.value.deref());

                        self.add_node_connections(&[
                            generics_node_root,
                            generic_node_root,
                            generic_node,
                        ]);
                    }

                    self.add_node_connections(&[root, generics_node_root]);
                }

                root
            }
            RawExpression::Index(lhs, rhs) => {
                let root = self.add_node("IndexExpr");

                let lhs_node = self.create_expression_node(lhs.value.deref());

                let rhs_node_root = self.add_node("Index");
                let rhs_node = self.create_expression_node(rhs.value.deref());

                self.add_node_connections(&[root, lhs_node]);
                self.add_node_connections(&[root, rhs_node_root, rhs_node]);

                root
            }
            RawExpression::Property(lhs, rhs) => {
                let root = self.add_node("PropertyAccess");

                let lhs_node = self.create_expression_node(lhs.value.deref());

                let rhs_node_root = self.add_node("Property");
                let rhs_node = self.add_node(&rhs.value);

                self.add_node_connections(&[root, lhs_node]);
                self.add_node_connections(&[root, rhs_node_root, rhs_node]);

                root
            }
            RawExpression::PrefixOrPostfix(t, e) => {
                let root = self.add_node("PrefixOrPostfix");

                let op_node = self.add_node(&t.value.to_string());
                let expr_node = self.create_expression_node(e.value.deref());

                self.add_node_connections(&[root, expr_node]);
                self.add_node_connections(&[root, op_node]);

                root
            }
            RawExpression::If(r#if, elseifs, r#else) => {
                let root = self.add_node("IfExpr");

                let if_node = self.add_node("If");
                let if_cond_node_root = self.add_node("Condition");
                let if_cond_node = self.create_expression_node(r#if.0.value.deref());
                let if_statement_block = self.create_statements_block_node(&r#if.1);

                self.add_node_connections(&[root, if_node, if_cond_node_root, if_cond_node]);
                self.add_node_connections(&[if_node, if_statement_block]);

                if !elseifs.is_empty() {
                    let elseifs_root = self.add_node("IfElseChain");

                    for elseif in elseifs {
                        let elseif_node = self.add_node("ElseIf");
                        let elseif_cond_node_root = self.add_node("Condition");
                        let elseif_cond_node = self.create_expression_node(elseif.0.value.deref());
                        let elseif_statement_block = self.create_statements_block_node(&elseif.1);

                        self.add_node_connections(&[
                            elseifs_root,
                            elseif_node,
                            elseif_cond_node_root,
                            elseif_cond_node,
                        ]);
                        self.add_node_connections(&[elseif_node, elseif_statement_block]);
                    }

                    self.add_node_connections(&[root, elseifs_root]);
                }

                if r#else.is_some() {
                    let else_node = self.add_node("Else");
                    let else_statement_block =
                        self.create_statements_block_node(r#else.as_ref().unwrap());

                    self.add_node_connections(&[root, else_node, else_statement_block]);
                }

                root
            }
            _ => todo!(),
        }
    }

    // Returns root node
    fn create_type_node(&mut self, r#type: &RawType) -> u32 {
        match r#type {
            RawType::Primary(p) => {
                let root = self.add_node("PrimaryType");
                let node = self.add_node(&p.value.to_string());

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Array(a) => {
                let root = self.add_node("ArrayType");
                let node = self.create_type_node(a.value.deref());

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Pointer(p) => {
                let root = self.add_node("PointerType");
                let node = self.create_type_node(p.value.deref());

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Custom(c, generics) => {
                let root = self.add_node("DefinedType");
                let node = self.add_node(&c.value);

                if !generics.is_empty() {
                    let generics_node_root = self.add_node("Generics");

                    for generic in generics {
                        let generic_node_root = self.add_node("Generic");
                        let generic_node = self.create_type_node(generic.value.deref());

                        self.add_node_connections(&[
                            generics_node_root,
                            generic_node_root,
                            generic_node,
                        ]);
                    }

                    self.add_node_connections(&[root, generics_node_root]);
                }

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Generic(g) => {
                let root = self.add_node("GenericType");
                let node = self.add_node(&g.value);

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Impls(t) => {
                let root = self.add_node("ImplsType");
                let node = self.create_type_node(t.value.deref());

                self.add_node_connections(&[root, node]);

                root
            }
            RawType::Option(t) => {
                let root = self.add_node("OptionType");
                let node = self.create_type_node(t.value.deref());

                self.add_node_connections(&[root, node]);

                root
            }
        }
    }

    fn add_node(&mut self, label: &str) -> u32 {
        self.current_node_index += 1;

        println!("\tnode{} [label=\"{}\"];", self.current_node_index, label);

        self.current_node_index
    }

    fn add_node_connections(&mut self, connections: &[u32]) {
        print!("\t");
        print!(
            "{}",
            &connections
                .iter()
                .map(|x| format!("node{x}"))
                .collect::<Vec<String>>()
                .join(" -> "),
        );
        println!(";");
    }
}
