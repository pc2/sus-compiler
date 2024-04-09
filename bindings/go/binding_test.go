package tree_sitter_sus_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-sus"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_sus.Language())
	if language == nil {
		t.Errorf("Error loading Sus grammar")
	}
}
