import React, { useState } from "react";

const CreateNewQuizBtn = ({ handler }: { handler: any }) => (
  <button name="create" onClick={handler}>
    New Quiz
  </button>
);

const CreateQuizForm = ({ children }: { children: any }) => (
  <form>{children}</form>
);

const Create = () => {
  const [pageState, setPageState] = useState("view");

  const handleCreateNewQuiz = (e: any) => {
    setPageState("creation");
  };

  return (
    <div>
      {/* If logged in, use user context to render with current quizzes */}
      {/* If no data found, display 'go here to get started' */}
      {/* Button to open Create Quiz form */}
      {/* 
        1). Quiz title
        2). Questions
            - question type
            - question body
            - button to add answer
            - answers/options
            - save answer button
        3). Save Quiz button
    */}
    </div>
  );
};

export default Create;
