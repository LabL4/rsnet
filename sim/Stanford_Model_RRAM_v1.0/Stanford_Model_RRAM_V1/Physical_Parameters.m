function varargout = Physical_Parameters(varargin)
% PHYSICAL_PARAMETERS MATLAB code for Physical_Parameters.fig
%      PHYSICAL_PARAMETERS, by itself, creates a new PHYSICAL_PARAMETERS or raises the existing
%      singleton*.
%
%      H = PHYSICAL_PARAMETERS returns the handle to a new PHYSICAL_PARAMETERS or the handle to
%      the existing singleton*.
%
%      PHYSICAL_PARAMETERS('CALLBACK',hObject,eventData,handles,...) calls the local
%      function named CALLBACK in PHYSICAL_PARAMETERS.M with the given input arguments.
%
%      PHYSICAL_PARAMETERS('Property','Value',...) creates a new PHYSICAL_PARAMETERS or raises the
%      existing singleton*.  Starting from the left, property value pairs are
%      applied to the GUI before Physical_Parameters_OpeningFcn gets called.  An
%      unrecognized property name or invalid value makes property application
%      stop.  All inputs are passed to Physical_Parameters_OpeningFcn via varargin.
%
%      *See GUI Options on GUIDE's Tools menu.  Choose "GUI allows only one
%      instance to run (singleton)".
%
% See also: GUIDE, GUIDATA, GUIHANDLES

% Edit the above text to modify the response to help Physical_Parameters

% Last Modified by GUIDE v2.5 24-Jul-2018 09:38:53

% Begin initialization code - DO NOT EDIT
gui_Singleton = 1;
gui_State = struct('gui_Name',       mfilename, ...
                   'gui_Singleton',  gui_Singleton, ...
                   'gui_OpeningFcn', @Physical_Parameters_OpeningFcn, ...
                   'gui_OutputFcn',  @Physical_Parameters_OutputFcn, ...
                   'gui_LayoutFcn',  [] , ...
                   'gui_Callback',   []);
if nargin && ischar(varargin{1})
    gui_State.gui_Callback = str2func(varargin{1});
end

if nargout
    [varargout{1:nargout}] = gui_mainfcn(gui_State, varargin{:});
else
    gui_mainfcn(gui_State, varargin{:});
end
% End initialization code - DO NOT EDIT


% --- Executes just before Physical_Parameters is made visible.
function Physical_Parameters_OpeningFcn(hObject, eventdata, handles, varargin)
% This function has no output args, see OutputFcn.
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
% varargin   command line arguments to Physical_Parameters (see VARARGIN)

% Choose default command line output for Physical_Parameters
handles.output = hObject;

% Update handles structure
guidata(hObject, handles);

% UIWAIT makes Physical_Parameters wait for user response (see UIRESUME)
% uiwait(handles.figure1);

global Parameters

Ea = Parameters.Physical.Ea; % Obtaining manually introduced value
set(handles.edit_Ea,'String',num2str(Ea)); % Transforming the value form string to double

a0 = Parameters.Physical.a0; % Obtaining manually introduced value
set(handles.edit_a0,'String',num2str(a0)); % Transforming the value form string to double


% --- Outputs from this function are returned to the command line.
function varargout = Physical_Parameters_OutputFcn(hObject, eventdata, handles) 
% varargout  cell array for returning output args (see VARARGOUT);
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Get default command line output from handles structure
varargout{1} = handles.output;



function edit_Ea_Callback(hObject, eventdata, handles)
% hObject    handle to edit_Ea (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_Ea as text
%        str2double(get(hObject,'String')) returns contents of edit_Ea as a double

global Parameters

Ea = get(handles.edit_Ea,'String'); % Obtaining manually introduced value
Ea = str2double(Ea); % Transforming the value form string to double

Parameters.Physical.Ea = Ea; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_Ea_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_Ea (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_a0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_a0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_a0 as text
%        str2double(get(hObject,'String')) returns contents of edit_a0 as a double

global Parameters

a0 = get(handles.edit_a0,'String'); % Obtaining manually introduced value
a0 = str2double(a0); % Transforming the value form string to double

Parameters.Physical.a0 = a0; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_a0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_a0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end


% --- Executes when user attempts to close figure1.
function figure1_CloseRequestFcn(hObject, eventdata, handles)
% hObject    handle to figure1 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hint: delete(hObject) closes the figure

global Parameters

save('Parameters.mat','Parameters');

delete(hObject);
