//! AI prompt templates for CV optimization

/// System prompt that establishes the AI as an HR expert
pub const SYSTEM_PROMPT: &str = r"You are an expert HR professional and recruitment specialist with over 20 years of experience. You have deep knowledge of:

1. Applicant Tracking Systems (ATS) and how to optimize CVs for them
2. What hiring managers look for in different industries
3. How to highlight relevant experience and skills
4. Best practices for CV structure and content

Your task is to analyze a CV and a job description, then provide a tailored version of the CV that:
- Emphasizes the most relevant experience and skills for the specific role
- Includes important keywords from the job description
- Maintains truthfulness while presenting information in the best light
- Follows modern CV best practices
- Is ATS-friendly";

/// User prompt template for CV tailoring
#[must_use]
pub fn create_user_prompt(cv_content: &str, job_description: &str) -> String {
    format!(
        r"Please analyze the following CV and job description, then provide an optimized version of the CV tailored specifically for this role.

CURRENT CV:
{cv_content}

JOB DESCRIPTION:
{job_description}

Please provide:
1. A professional summary tailored to this specific role
2. Experiences reordered and rewritten to emphasize relevant achievements
3. A prioritized list of skills that match the job requirements
4. Keywords from the job description that should be incorporated
5. Specific suggestions for further CV improvements

Remember to maintain honesty while presenting the candidate's experience in the most relevant way for this role."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_content() {
        // Test that system prompt contains expected content
        assert!(SYSTEM_PROMPT.contains("expert HR professional"));
        assert!(SYSTEM_PROMPT.contains("Applicant Tracking Systems"));
        assert!(SYSTEM_PROMPT.contains("ATS"));
        assert!(SYSTEM_PROMPT.contains("hiring managers"));
        assert!(SYSTEM_PROMPT.contains("CV structure"));
        assert!(SYSTEM_PROMPT.len() > 100); // Ensure it's substantial
    }

    #[test]
    fn test_create_user_prompt() {
        let cv_content = "John Doe\nSoftware Engineer\n10 years experience";
        let job_description = "Senior Developer position at Tech Corp";

        let prompt = create_user_prompt(cv_content, job_description);

        // Test that the prompt contains both inputs
        assert!(prompt.contains(cv_content));
        assert!(prompt.contains(job_description));

        // Test that it contains expected instructions
        assert!(prompt.contains("CURRENT CV:"));
        assert!(prompt.contains("JOB DESCRIPTION:"));
        assert!(prompt.contains("professional summary"));
        assert!(prompt.contains("reordered"));
        assert!(prompt.contains("skills"));
        assert!(prompt.contains("Keywords"));
        assert!(prompt.contains("suggestions"));
    }

    #[test]
    fn test_create_user_prompt_with_empty_inputs() {
        let prompt = create_user_prompt("", "");

        // Should still create a valid prompt structure
        assert!(prompt.contains("CURRENT CV:"));
        assert!(prompt.contains("JOB DESCRIPTION:"));
        assert!(!prompt.is_empty());
    }

    #[test]
    fn test_create_user_prompt_with_special_characters() {
        let cv_content = "Skills: C++, C#, .NET\nExperience: 5+ years";
        let job_description = "Looking for C++/C# developer with .NET experience";

        let prompt = create_user_prompt(cv_content, job_description);

        // Ensure special characters are preserved
        assert!(prompt.contains("C++"));
        assert!(prompt.contains("C#"));
        assert!(prompt.contains(".NET"));
        assert!(prompt.contains("5+"));
    }
}
