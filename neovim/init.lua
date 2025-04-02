--Show confirm-prompt
vim.opt.confirm = true

--Show Line Numbers
vim.opt.number = true

--Terminal Color
vim.opt.termguicolors = true

--Show Window Title
vim.opt.title = true

-- Tree Sitter Configuration
require('nvim-treesitter.configs').setup {
   -- Automatically install missing parsers when entering buffer
   auto_install = true,
   
   highlight = {
     -- Setting this to true will run `:h syntax` and tree-sitter at the same time.
     -- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
     -- Using this option may slow down your editor, and you may see some duplicate highlights.
     -- Instead of true it can also be a list of languages
     additional_vim_regex_highlighting = false,
     
     -- `false` will disable the whole extension
     enable = true,
   },

   incremental_selection = {
     enable = true,
     keymaps = {
       init_selection = "gnn",
       node_decremental = "grm",
       node_incremental = "grn",
       scope_incremental = "grc",
     },
   },

   indent = {
     enable = true
   },

   -- Install parsers synchronously (only applied to `ensure_installed`)
   sync_install = false,
     
   textobjects = { enable = true },
}

