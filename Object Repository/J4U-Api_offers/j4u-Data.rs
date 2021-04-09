<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>j4u-Data</name>
   <tag></tag>
   <elementGuidId>f5b7495b-926b-43b3-be3f-b32f43279d64</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \&quot;username\&quot; : \&quot;comj4u\&quot;,\n \&quot;password\&quot; : \&quot;j4u@456\&quot;,\n \&quot;MSISDN\&quot; : \&quot;243987654321\&quot;,\n \&quot;Category\&quot;: \&quot;Data\&quot;,\n \&quot;Channel\&quot; : \&quot;WEB\&quot;,\n \&quot;Language\&quot; : \&quot;en\&quot;,\n \&quot;RefNum\&quot; : \&quot;APP1528376639790\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>username</name>
      <type>Main</type>
      <value>comj4u</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>password</name>
      <type>Main</type>
      <value>j4u@456</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${j4u_Apioffer_Url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_Url</defaultValue>
      <description></description>
      <id>ea7ca15c-114a-4039-b54a-3b093dc09d13</id>
      <masked>false</masked>
      <name>j4u_Apioffer_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_target_msisdn</defaultValue>
      <description></description>
      <id>dc35415c-f113-43a2-a295-fe83b7ff68c5</id>
      <masked>false</masked>
      <name>j4u_Apioffer_target_msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_random_msisdn</defaultValue>
      <description></description>
      <id>b7080107-d890-40fe-bfb2-54cff76a8bff</id>
      <masked>false</masked>
      <name>j4u_Apioffer_random_msisdn</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
