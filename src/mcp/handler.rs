//! Tool name aliases for fixing model hallucinations.
//! Wraps ServerHandler to intercept call_tool and replace wrong names.

use std::borrow::Cow;
use std::future::Future;

use rmcp::ErrorData;
use rmcp::ServerHandler;
use rmcp::model::CallToolRequestParams;
use rmcp::model::CallToolResult;
use rmcp::model::ServerInfo;
use rmcp::model::Tool;
use rmcp::service::MaybeSendFuture;
use rmcp::service::RequestContext;
use rmcp::service::RoleServer;

/// Resolves hallucinated tool names to real ones.
pub fn resolve_tool_alias(name: &str) -> &str {
    match name {
        // === admin_sync ===
        "sync_index" => "admin_sync",
        "sync" => "admin_sync",
        "reindex" => "admin_sync",
        "refresh_index" => "admin_sync",
        "update_index" => "admin_sync",
        "rebuild" => "admin_sync",
        "rescan" => "admin_sync",
        "reload" => "admin_sync",

        // === get_file_content ===
        "get_file" => "get_file_content",
        "show_file" => "get_file_content",
        "open_file" => "get_file_content",
        "read_file" => "get_file_content",
        "view_file" => "get_file_content",
        "get_content" => "get_file_content",
        "file_content" => "get_file_content",
        "show_content" => "get_file_content",
        "cat" => "get_file_content",
        "getfile" => "get_file_content",

        // === get_file_ast ===
        "get_ast" => "get_file_ast",
        "show_ast" => "get_file_ast",
        "get_ast_data" => "get_file_ast",
        "file_ast" => "get_file_ast",
        "ast" => "get_file_ast",

        // === search_by_code_classes ===
        "search_by_code_traits" => "search_by_code_classes",
        "search_by_code_interfaces" => "search_by_code_classes",
        "search_by_code_structs" => "search_by_code_classes",
        "search_by_code_records" => "search_by_code_classes",
        "search_by_code_types" => "search_by_code_classes",
        "search_by_code_abstract" => "search_by_code_classes",
        "search_by_code_class" => "search_by_code_classes",
        "search_by_code_trait" => "search_by_code_classes",
        "search_by_code_interface" => "search_by_code_classes",
        "search_by_code_struct" => "search_by_code_classes",
        "search_by_code_record" => "search_by_code_classes",
        "search_by_code_type" => "search_by_code_classes",
        "search_by_code_implements" => "search_by_code_classes",
        "search_by_code_extends" => "search_by_code_classes",
        "search_by_code_inherits" => "search_by_code_classes",
        "search_by_code_object" => "search_by_code_classes",
        "search_by_code_prototype" => "search_by_code_classes",
        "search_by_code_blueprint" => "search_by_code_classes",
        "search_by_code_contract" => "search_by_code_classes",
        "search_by_code_protocol" => "search_by_code_classes",
        "search_classes" => "search_by_code_classes",
        "find_classes" => "search_by_code_classes",

        // === search_by_code_functions ===
        "search_by_code_methods" => "search_by_code_functions",
        "search_by_code_procedures" => "search_by_code_functions",
        "search_by_code_routines" => "search_by_code_functions",
        "search_by_code_def" => "search_by_code_functions",
        "search_by_code_fn" => "search_by_code_functions",
        "search_by_code_sub" => "search_by_code_functions",
        "search_by_code_func" => "search_by_code_functions",
        "search_by_code_fun" => "search_by_code_functions",
        "search_by_code_function" => "search_by_code_functions",
        "search_by_code_method" => "search_by_code_functions",
        "search_by_code_proc" => "search_by_code_functions",
        "search_by_code_lambda" => "search_by_code_functions",
        "search_by_code_closure" => "search_by_code_functions",
        "search_by_code_callback" => "search_by_code_functions",
        "search_by_code_handler" => "search_by_code_functions",
        "search_by_code_async" => "search_by_code_functions",
        "search_by_code_constructor" => "search_by_code_functions",
        "search_by_code_destructor" => "search_by_code_functions",
        "search_by_code_getter" => "search_by_code_functions",
        "search_by_code_setter" => "search_by_code_functions",
        "search_functions" => "search_by_code_functions",
        "find_functions" => "search_by_code_functions",

        // === search_by_code_imports ===
        "search_by_code_dependencies" => "search_by_code_imports",
        "search_by_code_libraries" => "search_by_code_imports",
        "search_by_code_modules" => "search_by_code_imports",
        "search_by_code_packages" => "search_by_code_imports",
        "search_by_code_requires" => "search_by_code_imports",
        "search_by_code_includes" => "search_by_code_imports",
        "search_by_code_using" => "search_by_code_imports",
        "search_by_code_use" => "search_by_code_imports",
        "search_by_code_import" => "search_by_code_imports",
        "search_by_code_deps" => "search_by_code_imports",
        "search_by_code_extern" => "search_by_code_imports",
        "search_imports" => "search_by_code_imports",
        "find_imports" => "search_by_code_imports",

        // === search_by_code_variables ===
        "search_by_code_enums" => "search_by_code_variables",
        "search_by_code_constants" => "search_by_code_variables",
        "search_by_code_globals" => "search_by_code_variables",
        "search_by_code_static" => "search_by_code_variables",
        "search_by_code_let" => "search_by_code_variables",
        "search_by_code_mutables" => "search_by_code_variables",
        "search_by_code_fields" => "search_by_code_variables",
        "search_by_code_variable" => "search_by_code_variables",
        "search_by_code_enum" => "search_by_code_variables",
        "search_by_code_constant" => "search_by_code_variables",
        "search_by_code_global" => "search_by_code_variables",
        "search_by_code_field" => "search_by_code_variables",
        "search_by_code_var" => "search_by_code_variables",
        "search_by_code_val" => "search_by_code_variables",
        "search_by_code_property" => "search_by_code_variables",
        "search_by_code_attribute" => "search_by_code_variables",
        "search_by_code_member" => "search_by_code_variables",
        "search_by_code_param" => "search_by_code_variables",
        "search_by_code_argument" => "search_by_code_variables",
        "search_by_code_binding" => "search_by_code_variables",
        "search_by_code_immutable" => "search_by_code_variables",
        "search_by_code_mut" => "search_by_code_variables",
        "search_by_code_ref" => "search_by_code_variables",
        "search_variables" => "search_by_code_variables",
        "find_variables" => "search_by_code_variables",

        // === show_projects ===
        "list_project_samples" => "show_projects",
        "list_sources" => "show_projects",
        "get_projects" => "show_projects",
        "list_all" => "show_projects",
        "projects" => "show_projects",
        "sources" => "show_projects",
        "list" => "show_projects",

        // === show_stats ===
        "stats" => "show_stats",
        "statistics" => "show_stats",
        "get_stats" => "show_stats",
        "project_stats" => "show_stats",
        "code_stats" => "show_stats",
        "file_count" => "show_stats",

        // === show_tree ===
        "show_structure" => "show_tree",
        "structure" => "show_tree",
        "tree" => "show_tree",
        "get_tree" => "show_tree",
        "view_structure" => "show_tree",
        "directory_tree" => "show_tree",
        "folder_structure" => "show_tree",
        "file_tree" => "show_tree",
        "list_files" => "show_tree",
        "browse" => "show_tree",
        "ls" => "show_tree",
        "dir" => "show_tree",

        // === search_documentation ===
        "search_by_docs" => "search_documentation",
        "show_docs" => "search_documentation",
        "search_docs" => "search_documentation",
        "find_docs" => "search_documentation",
        "search_by_documentation" => "search_documentation",
        "search_documentation" => "search_documentation",
        "show_documentation" => "search_documentation",
        "find_documentation" => "search_documentation",
        "get_documentation" => "search_documentation",
        "show_rules" => "search_documentation",
        "show_processes" => "search_documentation",
        "show_guides" => "search_documentation",
        "show_lore" => "search_documentation",
        "show_legends" => "search_documentation",
        "show_info" => "search_documentation",
        "get_info" => "search_documentation",
        "find_info" => "search_documentation",
        "search_info" => "search_documentation",
        "search_lore" => "search_documentation",
        "search_legends" => "search_documentation",
        "search_processes" => "search_documentation",

        // === search_knowledge ===
        "search_knowledge" => "search_knowledge",
        "find_knowledge" => "search_knowledge",
        "get_knowledge" => "search_knowledge",
        "show_knowledge" => "search_knowledge",
        "knowledge_search" => "search_knowledge",

        _ => name,
    }
}

/// Wraps a ServerHandler to add tool name aliases.
#[derive(Clone)]
pub struct AliasHandler<H>(pub H);

impl<H: ServerHandler> ServerHandler for AliasHandler<H> {
    /// Returns server info from the inner handler.
    fn get_info(&self) -> ServerInfo {
        self.0.get_info()
    }

    /// Resolves tool name aliases before lookup.
    fn get_tool(&self, name: &str) -> Option<Tool> {
        let resolved = resolve_tool_alias(name);
        if resolved != name {
            self.0.get_tool(resolved)
        } else {
            self.0.get_tool(name)
        }
    }

    /// Lists available tools from the inner handler.
    fn list_tools(
        &self,
        request: Option<rmcp::model::PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListToolsResult, ErrorData>> + MaybeSendFuture + '_
    {
        self.0.list_tools(request, context)
    }

    /// Resolves tool name aliases before calling the tool.
    fn call_tool(
        &self,
        mut request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, ErrorData>> + MaybeSendFuture + '_ {
        tracing::trace!("ALIAS_HANDLER: call_tool name={}", request.name);
        let resolved = resolve_tool_alias(request.name.as_ref());
        if resolved != request.name.as_ref() {
            tracing::trace!(
                "ALIAS_HANDLER: Resolving '{}' -> '{}'",
                request.name,
                resolved
            );
            request.name = Cow::Owned(resolved.to_string());
        }
        self.0.call_tool(request, context)
    }
}
