use anchor_lang::prelude::*;
use constants::{ANCHOR_DISCRIMINATOR, PUBKEY_SIZE, STRING_LENGTH_PREFIX};
mod constants;

declare_id!("92uUahvJkKWsZSfhqnoDH7CGjCpmSs3sN8FypfJdhJjy");

#[program]
pub mod anchor_student_intro {
    use constants::{MAX_INTRO_LENGTH, MAX_STUDENT_NAME_LENGTH};

    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        student_name: String,
        intro: String,
    ) -> Result<()> {
        // We require that the student name is not longer than 20 characters
        require!(
            student_name.len() <= MAX_STUDENT_NAME_LENGTH,
            StudentIntroError::StudentNameTooLong
        );

        // We require that the intro is not longer than 50 characters
        require!(
            intro.len() <= MAX_INTRO_LENGTH,
            StudentIntroError::IntroTooLong
        );

        msg!("Student Account Created");
        msg!("Student Name: {}", student_name);
        msg!("Intro: {}", intro);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.initializer.key();
        student_intro.student_name = student_name;
        student_intro.intro = intro;

        Ok(())
    }

    pub fn update_student_intro(
        ctx: Context<UpdateStudentIntro>,
        student_name: String,
        intro: String,
    ) -> Result<()> {
        // We require that the intro is not longer than 50 characters
        require!(
            intro.len() <= MAX_INTRO_LENGTH,
            StudentIntroError::IntroTooLong
        );

        msg!("Student Account Updated");
        msg!("Student Name: {}", student_name);
        msg!("Intro: {}", intro);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.intro = intro;

        Ok(())
    }

    pub fn delete_student_intro(
        _ctx: Context<DeleteStudentIntro>,
        student_name: String,
    ) -> Result<()> {
        msg!("Student Intro deleted");
        msg!("Student Name: {}", student_name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(student_name: String, intro: String)]
pub struct AddStudentIntro<'info> {
    #[account(init,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = StudentIntroState::INIT_SPACE + student_name.len() + intro.len()
    )]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student_name: String, intro: String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = StudentIntroState::INIT_SPACE + student_name.len() + intro.len(),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(student_name: String)]
pub struct DeleteStudentIntro<'info> {
    #[account(
        mut,
        seeds = [student_name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer)]
    pub student_intro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
}

#[account]
pub struct StudentIntroState {
    pub student: Pubkey,      //32
    pub student_name: String, //4 + len
    pub intro: String,        //4 + len
}

impl Space for StudentIntroState {
    const INIT_SPACE: usize =
        ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}

#[error_code]
enum StudentIntroError {
    #[msg("The student name is too long")]
    StudentNameTooLong,
    #[msg("The intro is too long")]
    IntroTooLong,
}
