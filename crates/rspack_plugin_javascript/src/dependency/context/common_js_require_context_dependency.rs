use rspack_core::{
  AsModuleDependency, Compilation, ContextDependency, ContextOptions, Dependency,
  DependencyCategory, DependencyId, DependencyTemplate, DependencyType, ModuleGraph,
  RealDependencyLocation, RuntimeSpec, TemplateContext, TemplateReplaceSource,
};
use rspack_error::Diagnostic;

use super::{
  context_dependency_template_as_require_call, create_resource_identifier_for_context_dependency,
};

#[derive(Debug, Clone)]
pub struct CommonJsRequireContextDependency {
  id: DependencyId,
  range: RealDependencyLocation,
  range_callee: RealDependencyLocation,
  resource_identifier: String,
  options: ContextOptions,
  optional: bool,
  critical: Option<Diagnostic>,
}

impl CommonJsRequireContextDependency {
  pub fn new(
    options: ContextOptions,
    range: RealDependencyLocation,
    range_callee: RealDependencyLocation,
    optional: bool,
  ) -> Self {
    let resource_identifier = create_resource_identifier_for_context_dependency(None, &options);
    Self {
      range,
      range_callee,
      options,
      resource_identifier,
      optional,
      id: DependencyId::new(),
      critical: None,
    }
  }
}

impl Dependency for CommonJsRequireContextDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }

  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::CommonJS
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::CommonJSRequireContext
  }

  fn range(&self) -> Option<&RealDependencyLocation> {
    Some(&self.range)
  }

  fn could_affect_referencing_module(&self) -> rspack_core::AffectType {
    rspack_core::AffectType::True
  }

  fn get_diagnostics(&self, _module_graph: &ModuleGraph) -> Option<Vec<Diagnostic>> {
    if let Some(critical) = self.critical() {
      return Some(vec![critical.clone()]);
    }
    None
  }
}

impl ContextDependency for CommonJsRequireContextDependency {
  fn request(&self) -> &str {
    &self.options.request
  }

  fn options(&self) -> &ContextOptions {
    &self.options
  }

  fn get_context(&self) -> Option<&str> {
    None
  }

  fn resource_identifier(&self) -> &str {
    &self.resource_identifier
  }

  fn set_request(&mut self, request: String) {
    self.options.request = request;
  }

  fn get_optional(&self) -> bool {
    self.optional
  }

  fn type_prefix(&self) -> rspack_core::ContextTypePrefix {
    rspack_core::ContextTypePrefix::Normal
  }

  fn critical(&self) -> &Option<Diagnostic> {
    &self.critical
  }

  fn critical_mut(&mut self) -> &mut Option<Diagnostic> {
    &mut self.critical
  }
}

impl DependencyTemplate for CommonJsRequireContextDependency {
  fn apply(
    &self,
    source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    context_dependency_template_as_require_call(
      self,
      source,
      code_generatable_context,
      self.range_callee.start,
      self.range_callee.end,
      self.range.end,
    );
  }

  fn dependency_id(&self) -> Option<DependencyId> {
    Some(self.id)
  }

  fn update_hash(
    &self,
    _hasher: &mut dyn std::hash::Hasher,
    _compilation: &Compilation,
    _runtime: Option<&RuntimeSpec>,
  ) {
  }
}

impl AsModuleDependency for CommonJsRequireContextDependency {}
