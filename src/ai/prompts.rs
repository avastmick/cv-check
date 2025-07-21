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
