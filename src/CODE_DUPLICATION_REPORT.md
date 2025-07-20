# Code Duplication Report

This document identifies code duplications in the CV Check codebase to help prevent accidental duplication and guide refactoring efforts.

## Summary

Several patterns of code duplication were found that could be refactored to improve maintainability and reduce the risk of inconsistencies.

## Major Duplications

### 1. Theme Loading Logic
**Location**: `themes/color.rs` and `themes/font.rs`
**Issue**: Identical pattern for loading themes by name with match statements
**Impact**: Changes to theme loading logic must be made in multiple places
**Solution**: Create a generic theme loader trait or function

### 2. Template File Reading
**Location**: All renderer modules (`pdf.rs`, `html.rs`, `docx.rs`)
**Issue**: Identical code for optionally reading template files
```rust
let template = if let Some(path) = template_path {
    Some(std::fs::read_to_string(path)?)
} else {
    None
};
```
**Impact**: Three copies of the same logic
**Solution**: Extract to a shared utility function in `render/mod.rs`

### 3. Markdown Parser Options
**Location**: `parser/markdown.rs:6-10` and `render/pdf.rs:184-188`
**Issue**: Identical markdown parsing configuration
**Impact**: Configuration drift if one is updated without the other
**Solution**: Create a shared function or constant for markdown options

### 4. Static String Allocations
**Location**: Throughout `themes/color.rs` and `themes/font.rs`
**Issue**: Using `.to_string()` on static string literals
**Impact**: Unnecessary allocations for values that never change
**Solution**: Use `&'static str` or create const values

### 5. Font Size Definitions
**Location**: `themes/font.rs` - all theme definitions
**Issue**: Same font sizes repeated across all themes (28pt, 16pt, 14pt, 11pt, 10pt)
**Impact**: Changing standard sizes requires updates in multiple places
**Solution**: Define size constants and reference them in themes

### 6. Default Theme Values
**Location**: Multiple files
- `config.rs:80-86` - default functions
- `config.rs:101-102` - GlobalConfig defaults
- `main.rs:32,36` - CLI argument defaults
**Issue**: "modern" default repeated in multiple places
**Impact**: Changing defaults requires finding all occurrences
**Solution**: Single source of truth for defaults

## Refactoring Recommendations

### High Priority
1. **Create Theme Trait**: Abstract common theme loading behavior
2. **Template Loader Utility**: Single function for optional template loading
3. **Markdown Options Builder**: Shared configuration for parser options

### Medium Priority
1. **Constants Module**: Define shared constants for:
   - Default theme names
   - Standard font sizes
   - Common file paths
2. **Static String Optimization**: Replace `.to_string()` with static references

### Low Priority
1. **Test Utilities**: Extract common test patterns for theme modules
2. **Error Builder**: Standardize error creation patterns

## Benefits of Refactoring

1. **Easier Maintenance**: Changes only need to be made in one place
2. **Consistency**: Reduces risk of drift between similar implementations
3. **Performance**: Fewer string allocations with static references
4. **Clarity**: Makes the codebase easier to understand
5. **Testing**: Shared utilities can be tested once

## Action Items

- [ ] Create issue for theme loading refactor
- [ ] Implement template loader utility
- [ ] Define constants module for shared values
- [ ] Optimize string allocations in theme definitions
- [ ] Extract markdown options to shared location
