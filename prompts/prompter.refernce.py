    def build_prompt(self, completion: str, feedback: str) -> str:
        """
        输入 Rust 源代码和编译反馈，返回拼接后的 Prompt
        """
        # 按照要求：Prompt + Rust Code + Feedback + Reminder 直接拼接
        parts = [self.prompt_template]
        
        if completion:
            parsed_completion = self._parse_response(completion) # 确保格式正确
            parts.append(parsed_completion)
        '''
        {{Feedback.compile}}
        ```bash
        // Compile Feedback
        ```
        '''
        if feedback:
            parts.append("{{Feedback.compile}}\n```bash\n" + feedback.strip() + "\n```")
        
        if self.reminder_content:
            parts.append(self.reminder_content)

        prompt = "\n".join(parts)
        return self._wrap_with_chat_template(prompt)