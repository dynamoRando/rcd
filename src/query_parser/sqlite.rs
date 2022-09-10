use antlr_rust::{
    common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory,
    tree::ParseTreeListener, InputStream,
};

use crate::rcd_enum::{DatabaseType, DmlType};

use self::{
    sqlitelexer::SQLiteLexer,
    sqlitelistener::SQLiteListener,
    sqliteparser::{SQLiteParser, SQLiteParserContext, SQLiteParserContextType},
};

mod sqlitelexer;
mod sqlitelistener;
mod sqliteparser;

#[derive(Clone, Debug)]
struct RcdSqliteListener {
    pub statement_type: Box<DmlData>,
}

#[derive(Copy, Clone, Debug)]
struct DmlData {
    pub data: DmlType,
}

impl<'input> ParseTreeListener<'input, SQLiteParserContextType> for RcdSqliteListener {
    fn enter_every_rule(&mut self, ctx: &dyn SQLiteParserContext<'input>) {
        println!(
            "rule entered {}",
            sqliteparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> SQLiteListener<'input> for RcdSqliteListener {
    fn enter_parse(&mut self, _ctx: &sqliteparser::ParseContext<'input>) {}

    fn exit_parse(&mut self, _ctx: &sqliteparser::ParseContext<'input>) {}

    fn enter_error(&mut self, _ctx: &sqliteparser::ErrorContext<'input>) {}

    fn exit_error(&mut self, _ctx: &sqliteparser::ErrorContext<'input>) {}

    fn enter_sql_stmt_list(&mut self, _ctx: &sqliteparser::Sql_stmt_listContext<'input>) {}

    fn exit_sql_stmt_list(&mut self, _ctx: &sqliteparser::Sql_stmt_listContext<'input>) {}

    fn enter_sql_stmt(&mut self, _ctx: &sqliteparser::Sql_stmtContext<'input>) {}

    fn exit_sql_stmt(&mut self, _ctx: &sqliteparser::Sql_stmtContext<'input>) {}

    fn enter_alter_table_stmt(&mut self, _ctx: &sqliteparser::Alter_table_stmtContext<'input>) {}

    fn exit_alter_table_stmt(&mut self, _ctx: &sqliteparser::Alter_table_stmtContext<'input>) {}

    fn enter_analyze_stmt(&mut self, _ctx: &sqliteparser::Analyze_stmtContext<'input>) {}

    fn exit_analyze_stmt(&mut self, _ctx: &sqliteparser::Analyze_stmtContext<'input>) {}

    fn enter_attach_stmt(&mut self, _ctx: &sqliteparser::Attach_stmtContext<'input>) {}

    fn exit_attach_stmt(&mut self, _ctx: &sqliteparser::Attach_stmtContext<'input>) {}

    fn enter_begin_stmt(&mut self, _ctx: &sqliteparser::Begin_stmtContext<'input>) {}

    fn exit_begin_stmt(&mut self, _ctx: &sqliteparser::Begin_stmtContext<'input>) {}

    fn enter_commit_stmt(&mut self, _ctx: &sqliteparser::Commit_stmtContext<'input>) {}

    fn exit_commit_stmt(&mut self, _ctx: &sqliteparser::Commit_stmtContext<'input>) {}

    fn enter_compound_select_stmt(
        &mut self,
        _ctx: &sqliteparser::Compound_select_stmtContext<'input>,
    ) {
    }

    fn exit_compound_select_stmt(
        &mut self,
        _ctx: &sqliteparser::Compound_select_stmtContext<'input>,
    ) {
    }

    fn enter_create_index_stmt(&mut self, _ctx: &sqliteparser::Create_index_stmtContext<'input>) {}

    fn exit_create_index_stmt(&mut self, _ctx: &sqliteparser::Create_index_stmtContext<'input>) {}

    fn enter_create_table_stmt(&mut self, _ctx: &sqliteparser::Create_table_stmtContext<'input>) {}

    fn exit_create_table_stmt(&mut self, _ctx: &sqliteparser::Create_table_stmtContext<'input>) {}

    fn enter_create_trigger_stmt(
        &mut self,
        _ctx: &sqliteparser::Create_trigger_stmtContext<'input>,
    ) {
    }

    fn exit_create_trigger_stmt(
        &mut self,
        _ctx: &sqliteparser::Create_trigger_stmtContext<'input>,
    ) {
    }

    fn enter_create_view_stmt(&mut self, _ctx: &sqliteparser::Create_view_stmtContext<'input>) {}

    fn exit_create_view_stmt(&mut self, _ctx: &sqliteparser::Create_view_stmtContext<'input>) {}

    fn enter_create_virtual_table_stmt(
        &mut self,
        _ctx: &sqliteparser::Create_virtual_table_stmtContext<'input>,
    ) {
    }

    fn exit_create_virtual_table_stmt(
        &mut self,
        _ctx: &sqliteparser::Create_virtual_table_stmtContext<'input>,
    ) {
    }

    fn enter_delete_stmt(&mut self, _ctx: &sqliteparser::Delete_stmtContext<'input>) {
        self.statement_type.data = DmlType::Delete;
        // println!("{:?}", self.statement_type);
        // println!("RCDSQLITELISTNER ENTERED DELETE STATEMENT");
    }

    fn exit_delete_stmt(&mut self, _ctx: &sqliteparser::Delete_stmtContext<'input>) {}

    fn enter_delete_stmt_limited(
        &mut self,
        _ctx: &sqliteparser::Delete_stmt_limitedContext<'input>,
    ) {
    }

    fn exit_delete_stmt_limited(
        &mut self,
        _ctx: &sqliteparser::Delete_stmt_limitedContext<'input>,
    ) {
    }

    fn enter_detach_stmt(&mut self, _ctx: &sqliteparser::Detach_stmtContext<'input>) {}

    fn exit_detach_stmt(&mut self, _ctx: &sqliteparser::Detach_stmtContext<'input>) {}

    fn enter_drop_index_stmt(&mut self, _ctx: &sqliteparser::Drop_index_stmtContext<'input>) {}

    fn exit_drop_index_stmt(&mut self, _ctx: &sqliteparser::Drop_index_stmtContext<'input>) {}

    fn enter_drop_table_stmt(&mut self, _ctx: &sqliteparser::Drop_table_stmtContext<'input>) {}

    fn exit_drop_table_stmt(&mut self, _ctx: &sqliteparser::Drop_table_stmtContext<'input>) {}

    fn enter_drop_trigger_stmt(&mut self, _ctx: &sqliteparser::Drop_trigger_stmtContext<'input>) {}

    fn exit_drop_trigger_stmt(&mut self, _ctx: &sqliteparser::Drop_trigger_stmtContext<'input>) {}

    fn enter_drop_view_stmt(&mut self, _ctx: &sqliteparser::Drop_view_stmtContext<'input>) {}

    fn exit_drop_view_stmt(&mut self, _ctx: &sqliteparser::Drop_view_stmtContext<'input>) {}

    fn enter_factored_select_stmt(
        &mut self,
        _ctx: &sqliteparser::Factored_select_stmtContext<'input>,
    ) {
    }

    fn exit_factored_select_stmt(
        &mut self,
        _ctx: &sqliteparser::Factored_select_stmtContext<'input>,
    ) {
    }

    fn enter_insert_stmt(&mut self, _ctx: &sqliteparser::Insert_stmtContext<'input>) {
        self.statement_type.data = DmlType::Insert;
        // println!("{:?}", self.statement_type);
        // println!("RCDSQLITELISTNER ENTERED INSERT STATEMENT");
    }

    fn exit_insert_stmt(&mut self, _ctx: &sqliteparser::Insert_stmtContext<'input>) {}

    fn enter_pragma_stmt(&mut self, _ctx: &sqliteparser::Pragma_stmtContext<'input>) {}

    fn exit_pragma_stmt(&mut self, _ctx: &sqliteparser::Pragma_stmtContext<'input>) {}

    fn enter_reindex_stmt(&mut self, _ctx: &sqliteparser::Reindex_stmtContext<'input>) {}

    fn exit_reindex_stmt(&mut self, _ctx: &sqliteparser::Reindex_stmtContext<'input>) {}

    fn enter_release_stmt(&mut self, _ctx: &sqliteparser::Release_stmtContext<'input>) {}

    fn exit_release_stmt(&mut self, _ctx: &sqliteparser::Release_stmtContext<'input>) {}

    fn enter_rollback_stmt(&mut self, _ctx: &sqliteparser::Rollback_stmtContext<'input>) {}

    fn exit_rollback_stmt(&mut self, _ctx: &sqliteparser::Rollback_stmtContext<'input>) {}

    fn enter_savepoint_stmt(&mut self, _ctx: &sqliteparser::Savepoint_stmtContext<'input>) {}

    fn exit_savepoint_stmt(&mut self, _ctx: &sqliteparser::Savepoint_stmtContext<'input>) {}

    fn enter_simple_select_stmt(&mut self, _ctx: &sqliteparser::Simple_select_stmtContext<'input>) {
    }

    fn exit_simple_select_stmt(&mut self, _ctx: &sqliteparser::Simple_select_stmtContext<'input>) {}

    fn enter_select_stmt(&mut self, _ctx: &sqliteparser::Select_stmtContext<'input>) {
        self.statement_type.data = DmlType::Select;
        // println!("{:?}", self.statement_type);
        // println!("RCDSQLITELISTNER ENTERED SELECT STATEMENT");
    }

    fn exit_select_stmt(&mut self, _ctx: &sqliteparser::Select_stmtContext<'input>) {}

    fn enter_select_or_values(&mut self, _ctx: &sqliteparser::Select_or_valuesContext<'input>) {}

    fn exit_select_or_values(&mut self, _ctx: &sqliteparser::Select_or_valuesContext<'input>) {}

    fn enter_update_stmt(&mut self, _ctx: &sqliteparser::Update_stmtContext<'input>) {
        self.statement_type.data = DmlType::Update;
        // println!("{:?}", self.statement_type);
        // println!("RCDSQLITELISTNER ENTERED UPDATE STATEMENT");
    }

    fn exit_update_stmt(&mut self, _ctx: &sqliteparser::Update_stmtContext<'input>) {}

    fn enter_update_stmt_limited(
        &mut self,
        _ctx: &sqliteparser::Update_stmt_limitedContext<'input>,
    ) {
    }

    fn exit_update_stmt_limited(
        &mut self,
        _ctx: &sqliteparser::Update_stmt_limitedContext<'input>,
    ) {
    }

    fn enter_vacuum_stmt(&mut self, _ctx: &sqliteparser::Vacuum_stmtContext<'input>) {}

    fn exit_vacuum_stmt(&mut self, _ctx: &sqliteparser::Vacuum_stmtContext<'input>) {}

    fn enter_column_def(&mut self, _ctx: &sqliteparser::Column_defContext<'input>) {}

    fn exit_column_def(&mut self, _ctx: &sqliteparser::Column_defContext<'input>) {}

    fn enter_type_name(&mut self, _ctx: &sqliteparser::Type_nameContext<'input>) {}

    fn exit_type_name(&mut self, _ctx: &sqliteparser::Type_nameContext<'input>) {}

    fn enter_column_constraint(&mut self, _ctx: &sqliteparser::Column_constraintContext<'input>) {}

    fn exit_column_constraint(&mut self, _ctx: &sqliteparser::Column_constraintContext<'input>) {}

    fn enter_conflict_clause(&mut self, _ctx: &sqliteparser::Conflict_clauseContext<'input>) {}

    fn exit_conflict_clause(&mut self, _ctx: &sqliteparser::Conflict_clauseContext<'input>) {}

    fn enter_expr(&mut self, _ctx: &sqliteparser::ExprContext<'input>) {}

    fn exit_expr(&mut self, _ctx: &sqliteparser::ExprContext<'input>) {}

    fn enter_foreign_key_clause(&mut self, _ctx: &sqliteparser::Foreign_key_clauseContext<'input>) {
    }

    fn exit_foreign_key_clause(&mut self, _ctx: &sqliteparser::Foreign_key_clauseContext<'input>) {}

    fn enter_raise_function(&mut self, _ctx: &sqliteparser::Raise_functionContext<'input>) {}

    fn exit_raise_function(&mut self, _ctx: &sqliteparser::Raise_functionContext<'input>) {}

    fn enter_indexed_column(&mut self, _ctx: &sqliteparser::Indexed_columnContext<'input>) {}

    fn exit_indexed_column(&mut self, _ctx: &sqliteparser::Indexed_columnContext<'input>) {}

    fn enter_table_constraint(&mut self, _ctx: &sqliteparser::Table_constraintContext<'input>) {}

    fn exit_table_constraint(&mut self, _ctx: &sqliteparser::Table_constraintContext<'input>) {}

    fn enter_with_clause(&mut self, _ctx: &sqliteparser::With_clauseContext<'input>) {}

    fn exit_with_clause(&mut self, _ctx: &sqliteparser::With_clauseContext<'input>) {}

    fn enter_qualified_table_name(
        &mut self,
        _ctx: &sqliteparser::Qualified_table_nameContext<'input>,
    ) {
    }

    fn exit_qualified_table_name(
        &mut self,
        _ctx: &sqliteparser::Qualified_table_nameContext<'input>,
    ) {
    }

    fn enter_ordering_term(&mut self, _ctx: &sqliteparser::Ordering_termContext<'input>) {}

    fn exit_ordering_term(&mut self, _ctx: &sqliteparser::Ordering_termContext<'input>) {}

    fn enter_pragma_value(&mut self, _ctx: &sqliteparser::Pragma_valueContext<'input>) {}

    fn exit_pragma_value(&mut self, _ctx: &sqliteparser::Pragma_valueContext<'input>) {}

    fn enter_common_table_expression(
        &mut self,
        _ctx: &sqliteparser::Common_table_expressionContext<'input>,
    ) {
    }

    fn exit_common_table_expression(
        &mut self,
        _ctx: &sqliteparser::Common_table_expressionContext<'input>,
    ) {
    }

    fn enter_result_column(&mut self, _ctx: &sqliteparser::Result_columnContext<'input>) {}

    fn exit_result_column(&mut self, _ctx: &sqliteparser::Result_columnContext<'input>) {}

    fn enter_table_or_subquery(&mut self, _ctx: &sqliteparser::Table_or_subqueryContext<'input>) {}

    fn exit_table_or_subquery(&mut self, _ctx: &sqliteparser::Table_or_subqueryContext<'input>) {}

    fn enter_join_clause(&mut self, _ctx: &sqliteparser::Join_clauseContext<'input>) {}

    fn exit_join_clause(&mut self, _ctx: &sqliteparser::Join_clauseContext<'input>) {}

    fn enter_join_operator(&mut self, _ctx: &sqliteparser::Join_operatorContext<'input>) {}

    fn exit_join_operator(&mut self, _ctx: &sqliteparser::Join_operatorContext<'input>) {}

    fn enter_join_constraint(&mut self, _ctx: &sqliteparser::Join_constraintContext<'input>) {}

    fn exit_join_constraint(&mut self, _ctx: &sqliteparser::Join_constraintContext<'input>) {}

    fn enter_select_core(&mut self, _ctx: &sqliteparser::Select_coreContext<'input>) {
        self.statement_type.data = DmlType::Select;
        // println!("{:?}", self.statement_type);
        // println!("RCDSQLITELISTNER ENTERED SELECT CORE STATEMENT");
    }

    fn exit_select_core(&mut self, _ctx: &sqliteparser::Select_coreContext<'input>) {}

    fn enter_compound_operator(&mut self, _ctx: &sqliteparser::Compound_operatorContext<'input>) {}

    fn exit_compound_operator(&mut self, _ctx: &sqliteparser::Compound_operatorContext<'input>) {}

    fn enter_signed_number(&mut self, _ctx: &sqliteparser::Signed_numberContext<'input>) {}

    fn exit_signed_number(&mut self, _ctx: &sqliteparser::Signed_numberContext<'input>) {}

    fn enter_literal_value(&mut self, _ctx: &sqliteparser::Literal_valueContext<'input>) {}

    fn exit_literal_value(&mut self, _ctx: &sqliteparser::Literal_valueContext<'input>) {}

    fn enter_unary_operator(&mut self, _ctx: &sqliteparser::Unary_operatorContext<'input>) {}

    fn exit_unary_operator(&mut self, _ctx: &sqliteparser::Unary_operatorContext<'input>) {}

    fn enter_error_message(&mut self, _ctx: &sqliteparser::Error_messageContext<'input>) {}

    fn exit_error_message(&mut self, _ctx: &sqliteparser::Error_messageContext<'input>) {}

    fn enter_module_argument(&mut self, _ctx: &sqliteparser::Module_argumentContext<'input>) {}

    fn exit_module_argument(&mut self, _ctx: &sqliteparser::Module_argumentContext<'input>) {}

    fn enter_column_alias(&mut self, _ctx: &sqliteparser::Column_aliasContext<'input>) {}

    fn exit_column_alias(&mut self, _ctx: &sqliteparser::Column_aliasContext<'input>) {}

    fn enter_keyword(&mut self, _ctx: &sqliteparser::KeywordContext<'input>) {}

    fn exit_keyword(&mut self, _ctx: &sqliteparser::KeywordContext<'input>) {}

    fn enter_name(&mut self, _ctx: &sqliteparser::NameContext<'input>) {}

    fn exit_name(&mut self, _ctx: &sqliteparser::NameContext<'input>) {}

    fn enter_function_name(&mut self, _ctx: &sqliteparser::Function_nameContext<'input>) {}

    fn exit_function_name(&mut self, _ctx: &sqliteparser::Function_nameContext<'input>) {}

    fn enter_database_name(&mut self, _ctx: &sqliteparser::Database_nameContext<'input>) {}

    fn exit_database_name(&mut self, _ctx: &sqliteparser::Database_nameContext<'input>) {}

    fn enter_schema_name(&mut self, _ctx: &sqliteparser::Schema_nameContext<'input>) {}

    fn exit_schema_name(&mut self, _ctx: &sqliteparser::Schema_nameContext<'input>) {}

    fn enter_table_function_name(
        &mut self,
        _ctx: &sqliteparser::Table_function_nameContext<'input>,
    ) {
    }

    fn exit_table_function_name(
        &mut self,
        _ctx: &sqliteparser::Table_function_nameContext<'input>,
    ) {
    }

    fn enter_table_name(&mut self, _ctx: &sqliteparser::Table_nameContext<'input>) {
        // println!("SQLiteListener ENTERED TABLE NAME");
        // println!("{:?}", _ctx.start().text);
    }

    fn exit_table_name(&mut self, _ctx: &sqliteparser::Table_nameContext<'input>) {}

    fn enter_table_or_index_name(
        &mut self,
        _ctx: &sqliteparser::Table_or_index_nameContext<'input>,
    ) {
    }

    fn exit_table_or_index_name(
        &mut self,
        _ctx: &sqliteparser::Table_or_index_nameContext<'input>,
    ) {
    }

    fn enter_new_table_name(&mut self, _ctx: &sqliteparser::New_table_nameContext<'input>) {}

    fn exit_new_table_name(&mut self, _ctx: &sqliteparser::New_table_nameContext<'input>) {}

    fn enter_column_name(&mut self, _ctx: &sqliteparser::Column_nameContext<'input>) {
        // println!("SQLiteListener ENTERED COLUMN NAME");
        // println!("{:?}", _ctx.start().text);
    }

    fn exit_column_name(&mut self, _ctx: &sqliteparser::Column_nameContext<'input>) {}

    fn enter_collation_name(&mut self, _ctx: &sqliteparser::Collation_nameContext<'input>) {}

    fn exit_collation_name(&mut self, _ctx: &sqliteparser::Collation_nameContext<'input>) {}

    fn enter_foreign_table(&mut self, _ctx: &sqliteparser::Foreign_tableContext<'input>) {}

    fn exit_foreign_table(&mut self, _ctx: &sqliteparser::Foreign_tableContext<'input>) {}

    fn enter_index_name(&mut self, _ctx: &sqliteparser::Index_nameContext<'input>) {}

    fn exit_index_name(&mut self, _ctx: &sqliteparser::Index_nameContext<'input>) {}

    fn enter_trigger_name(&mut self, _ctx: &sqliteparser::Trigger_nameContext<'input>) {}

    fn exit_trigger_name(&mut self, _ctx: &sqliteparser::Trigger_nameContext<'input>) {}

    fn enter_view_name(&mut self, _ctx: &sqliteparser::View_nameContext<'input>) {}

    fn exit_view_name(&mut self, _ctx: &sqliteparser::View_nameContext<'input>) {}

    fn enter_module_name(&mut self, _ctx: &sqliteparser::Module_nameContext<'input>) {}

    fn exit_module_name(&mut self, _ctx: &sqliteparser::Module_nameContext<'input>) {}

    fn enter_pragma_name(&mut self, _ctx: &sqliteparser::Pragma_nameContext<'input>) {}

    fn exit_pragma_name(&mut self, _ctx: &sqliteparser::Pragma_nameContext<'input>) {}

    fn enter_savepoint_name(&mut self, _ctx: &sqliteparser::Savepoint_nameContext<'input>) {}

    fn exit_savepoint_name(&mut self, _ctx: &sqliteparser::Savepoint_nameContext<'input>) {}

    fn enter_table_alias(&mut self, _ctx: &sqliteparser::Table_aliasContext<'input>) {}

    fn exit_table_alias(&mut self, _ctx: &sqliteparser::Table_aliasContext<'input>) {}

    fn enter_transaction_name(&mut self, _ctx: &sqliteparser::Transaction_nameContext<'input>) {}

    fn exit_transaction_name(&mut self, _ctx: &sqliteparser::Transaction_nameContext<'input>) {}

    fn enter_any_name(&mut self, _ctx: &sqliteparser::Any_nameContext<'input>) {}

    fn exit_any_name(&mut self, _ctx: &sqliteparser::Any_nameContext<'input>) {}
}

#[allow(dead_code, unused_variables)]
pub fn get_table_name(cmd: &str, db_type: DatabaseType) -> String {
    unimplemented!()
}

#[allow(dead_code, unused_variables, unused_mut)]
pub fn determine_statement_type(sql_text: String) -> DmlType {
    let text = sql_text.as_str();

    // println!("{}", sql_text);

    let tf = CommonTokenFactory::default();
    let input = InputStream::new(text.into());
    let mut lexer = SQLiteLexer::new_with_token_factory(input, &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SQLiteParser::new(token_source);

    let rcd_listener = RcdSqliteListener {
        statement_type: Box::new(DmlData {
            data: DmlType::Unknown,
        }),
    };

    let listener_id = parser.add_parse_listener(Box::new(rcd_listener));
    let result = parser.parse();
    let item = parser.remove_parse_listener(listener_id);

    return item.statement_type.data;
}
