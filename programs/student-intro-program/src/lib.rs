use anchor_lang::prelude::*;

declare_id!("Your Declare Id");

#[program]
pub mod student_intro_program {
    

    use super::*;

    pub fn add_student_intro( 
        ctx:Context<AddStudentIntro>,
        name:String,
        message:String,
    ) ->Result<()>{
        msg!("Student cREATED");
        msg!("Name: {}", name);
        msg!("Message: {}", message);
        

        let student_ıntro = &mut ctx.accounts.student_ıntro;
        student_ıntro.student=ctx.accounts.student.key();
        student_ıntro.name=name;
        student_ıntro.message=message;

        Ok(())

    }


    pub fn update_student_intro(
        ctx:Context<UpdateStudentIntro>,
        name:String,
        message:String

    ) -> Result<()>{
        msg!("Student update");
        msg!("Name {}", name);
        msg!("Message {}", message);

        let student_ıntro = &mut ctx.accounts.student_ıntro;
       student_ıntro.student = ctx.accounts.student.key();
        student_ıntro.name = name;
        student_ıntro.message = message;

        Ok(())

    }

    pub fn delete_student_intro(
        _ctx:Context<DeleteStudentIntro>,
         name:String
    ) ->Result<()>{
        msg!("Student delete {}", name);
        Ok(())

    }




   
}


#[account]
pub struct StudentIntroState{
    pub student:Pubkey,
    pub name:String,
    pub message:String

}


#[derive(Accounts)]
#[instruction(name:String, message:String)]

pub struct AddStudentIntro<'info>{
    #[account(
        init,
        seeds = [name.as_bytes(), student.key().as_ref()],
        bump,
        payer = student,
        space = 8 + 32 + 4  + name.len() + 4 + message.len()
    )]

    pub student_ıntro:Account<'info, StudentIntroState>,
    #[account(mut)]
    pub student : Signer<'info>,
    pub system_program:Program<'info, System>

}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct  UpdateStudentIntro<'info>{
    #[account(
        mut,
        seeds=[name.as_bytes(), student.key().as_ref()],
        bump,
        realloc= 8 + 32 + 4 + name.len() + 4  + message.len(),
        realloc::payer = student,
        realloc::zero= true,

    )]

    pub student_ıntro: Account<'info, StudentIntroState>,
    #[account(mut)]
    pub student:Signer<'info>,
    pub system_program: Program<'info, System>

}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct DeleteStudentIntro<'info>{
    #[account(
        mut,
        seeds= [name.as_bytes(), student.key().as_ref()],
        bump,
        close=student

    )]
    pub student_ıntro : Account<'info, StudentIntroState>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program:Program<'info, System>

}