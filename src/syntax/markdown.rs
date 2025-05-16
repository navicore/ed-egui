use std::collections::HashMap;
use egui::{Context, text::LayoutJob, FontId, TextFormat};
use crate::syntax::{ContentBlock, HighlightTheme, SyntaxHighlighter, TokenType};

/// Highlighter for Markdown content with embedded code blocks
pub struct MarkdownHighlighter {
    theme: HighlightTheme,
    language_highlighters: HashMap<String, Box<dyn SyntaxHighlighter>>,
}

impl MarkdownHighlighter {
    pub fn new() -> Self {
        Self {
            theme: HighlightTheme::default(),
            language_highlighters: HashMap::new(),
        }
    }
    
    fn parse_blocks(&self, text: &str) -> Vec<ContentBlock> {
        let mut blocks = Vec::new();
        let mut current_pos = 0;
        let mut in_code_block = false;
        let mut code_block_language = None;
        let mut code_block_start = 0;
        
        for (line_idx, line) in text.lines().enumerate() {
            let line_pos = current_pos;
            let line_len = line.len() + 1; // +1 for newline
            
            // Check for code block delimiter
            if line.trim().starts_with("```") {
                if in_code_block {
                    // End of code block
                    blocks.push(ContentBlock {
                        start: code_block_start,
                        end: line_pos + line_len,
                        language: code_block_language.clone(),
                        is_code_block: true,
                    });
                    
                    in_code_block = false;
                    code_block_language = None;
                } else {
                    // Start of code block
                    // If there's content before this code block, add it as markdown
                    if code_block_start < line_pos && !blocks.is_empty() {
                        let last_block_end = blocks.last().map_or(0, |b| b.end);
                        if last_block_end < line_pos {
                            blocks.push(ContentBlock {
                                start: last_block_end,
                                end: line_pos,
                                language: Some("markdown".to_string()),
                                is_code_block: false,
                            });
                        }
                    }
                    
                    // Parse language from fence
                    let fence_content = line.trim().strip_prefix("```").unwrap_or("");
                    if !fence_content.is_empty() {
                        code_block_language = Some(fence_content.to_string());
                    }
                    
                    code_block_start = line_pos;
                    in_code_block = true;
                }
            }
            
            current_pos += line_len;
        }
        
        // Add any remaining content
        if current_pos > 0 {
            let last_block_end = blocks.last().map_or(0, |b| b.end);
            if last_block_end < current_pos {
                blocks.push(ContentBlock {
                    start: last_block_end,
                    end: current_pos,
                    language: Some(if in_code_block { 
                        code_block_language.unwrap_or_else(|| "text".to_string()) 
                    } else { 
                        "markdown".to_string() 
                    }),
                    is_code_block: in_code_block,
                });
            }
        }
        
        blocks
    }
    
    fn highlight_markdown(&self, text: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        let mut pos = 0;
        
        // Simple and incomplete markdown highlighting
        for line in text.lines() {
            let line_start = pos;
            let line_len = line.len();
            
            // Headings
            if line.starts_with('#') {
                let mut level = 0;
                for c in line.chars() {
                    if c == '#' {
                        level += 1;
                    } else {
                        break;
                    }
                }
                
                if level > 0 && level <= 6 && (line.chars().nth(level) == Some(' ')) {
                    // Add the heading markers
                    job.append(
                        &line[0..level],
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.heading[level.min(6) - 1],
                            ..Default::default()
                        },
                    );
                    
                    // Add the heading text
                    job.append(
                        &line[level..],
                        0.0,
                        TextFormat {
                            font_id: FontId::proportional(16.0 + (6 - level) as f32),
                            color: self.theme.heading[level.min(6) - 1],
                            ..Default::default()
                        },
                    );
                    
                    job.append(
                        "\n",
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.foreground,
                            ..Default::default()
                        },
                    );
                    
                    pos += line_len + 1;
                    continue;
                }
            }
            
            // Bold/Strong (very simple implementation)
            if line.contains("**") {
                let parts: Vec<&str> = line.split("**").collect();
                let mut is_bold = false;
                
                for (i, part) in parts.iter().enumerate() {
                    let format = if is_bold {
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.strong,
                            italics: false,
                            ..Default::default()
                        }
                    } else {
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.foreground,
                            ..Default::default()
                        }
                    };
                    
                    job.append(part, 0.0, format);
                    
                    // Add the delimiter except for the last part
                    if i < parts.len() - 1 {
                        if !is_bold {
                            // Opening delimiter
                            job.append(
                                "**",
                                0.0,
                                TextFormat {
                                    font_id: FontId::monospace(14.0),
                                    color: self.theme.operator,
                                    ..Default::default()
                                },
                            );
                        } else {
                            // Closing delimiter
                            job.append(
                                "**",
                                0.0,
                                TextFormat {
                                    font_id: FontId::monospace(14.0),
                                    color: self.theme.operator,
                                    ..Default::default()
                                },
                            );
                        }
                        is_bold = !is_bold;
                    }
                }
                
                job.append(
                    "\n",
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.foreground,
                        ..Default::default()
                    },
                );
                
                pos += line_len + 1;
                continue;
            }
            
            // Lists (simple implementation)
            if line.trim().starts_with("- ") || line.trim().starts_with("* ") {
                let indent_len = line.len() - line.trim_start().len();
                let marker_len = 2; // "- " or "* "
                
                // Add any indentation
                if indent_len > 0 {
                    job.append(
                        &line[0..indent_len],
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.foreground,
                            ..Default::default()
                        },
                    );
                }
                
                // Add the list marker
                job.append(
                    &line[indent_len..(indent_len + marker_len)],
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.list,
                        ..Default::default()
                    },
                );
                
                // Add the list text
                job.append(
                    &line[(indent_len + marker_len)..],
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.foreground,
                        ..Default::default()
                    },
                );
                
                job.append(
                    "\n",
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.foreground,
                        ..Default::default()
                    },
                );
                
                pos += line_len + 1;
                continue;
            }
            
            // Default formatting for other lines
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(14.0),
                    color: self.theme.foreground,
                    ..Default::default()
                },
            );
            
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: FontId::monospace(14.0),
                    color: self.theme.foreground,
                    ..Default::default()
                },
            );
            
            pos += line_len + 1;
        }
        
        job
    }
    
    fn highlight_code_block(&self, text: &str, language: Option<&str>) -> LayoutJob {
        let mut job = LayoutJob::default();
        
        // Simple code highlighting for now
        // In a real implementation, you'd invoke the appropriate language highlighter
        job.append(
            text,
            0.0,
            TextFormat {
                font_id: FontId::monospace(14.0),
                color: self.theme.code_block,
                background: self.theme.background,
                ..Default::default()
            },
        );
        
        job
    }
}

impl SyntaxHighlighter for MarkdownHighlighter {
    fn highlight(&self, ctx: &Context, text: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        
        // Parse content blocks (markdown and code)
        let blocks = self.parse_blocks(text);
        
        for block in blocks {
            let block_text = &text[block.start..block.end];
            
            if block.is_code_block {
                // Handle code block
                let fence_line_end = block_text.find('\n').unwrap_or(block_text.len());
                let fence_line = &block_text[0..fence_line_end];
                
                // Add the opening fence with language
                job.append(
                    fence_line,
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.operator,
                        ..Default::default()
                    },
                );
                
                job.append(
                    "\n",
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(14.0),
                        color: self.theme.foreground,
                        ..Default::default()
                    },
                );
                
                // Handle the code block content
                let content_start = fence_line_end + 1;
                let content_end = block_text.rfind("```").unwrap_or(block_text.len());
                if content_start < content_end {
                    let code_content = &block_text[content_start..content_end];
                    
                    // For now, just use a simple highlighting
                    // In a real implementation, you'd use the appropriate language highlighter
                    job.append(
                        code_content,
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.foreground,
                            background: self.theme.background.linear_multiply(0.5),
                            ..Default::default()
                        },
                    );
                }
                
                // Add the closing fence
                if content_end < block_text.len() {
                    job.append(
                        &block_text[content_end..],
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(14.0),
                            color: self.theme.operator,
                            ..Default::default()
                        },
                    );
                }
            } else {
                // Handle markdown content
                let markdown_job = self.highlight_markdown(block_text);
                job.append_job(markdown_job);
            }
        }
        
        job
    }
    
    fn set_theme(&mut self, theme: HighlightTheme) {
        self.theme = theme;
    }
    
    fn theme(&self) -> &HighlightTheme {
        &self.theme
    }
}