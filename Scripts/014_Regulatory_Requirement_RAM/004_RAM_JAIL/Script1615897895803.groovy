import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WS.delay(10)

Old_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

Old_Profile_ID = WS.getElementPropertyValue(Old_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', FailureHandling.CONTINUE_ON_FAILURE)

println(Old_Profile_ID)

WS.comment('This above profile is old profile')

GlobalVariable.RAM_Bundle = PRODUCT_ID

GlobalVariable.RAM_SUBCOSID = SUBCOSID

WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/04_JAIL_Bundle purchase'))

WS.delay(15)

JAIL_Profile = WS.sendRequest(findTestObject('REGULATORY_REQUIREMENT/01_RAM_Queryprofile'))

JAIL_Profile_ID = WS.getElementPropertyValue(JAIL_Profile, 'QuerySubCosIDResultMsg.QuerySubCosIDResult', FailureHandling.CONTINUE_ON_FAILURE)

println(JAIL_Profile_ID)

WS.comment('This above profile is JAIL profile id')

